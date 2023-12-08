import React from "react";
import MonacoEditor, { Monaco } from "@monaco-editor/react";
import { ProjectManager } from "../../project-manager";
import * as monaco from "monaco-editor";
import {
  extractContractImplMethods,
  findContractEvents,
  findContractLineNumber,
} from "../../parser";

interface Deploy {
  type: "deploy";
  params: object;
}

interface Invoke {
  type: "invoke";
  params: object;
}

interface Subscribe {
  type: "subscription";
  params: object;
}

export type Command = Deploy | Invoke | Subscribe;

interface EditorConfig {
  multiFile: boolean;
  editable: boolean;
}

export interface EditorProps {
  config: EditorConfig;
  fileId: number;
  manager: ProjectManager;
  onEvent: (command: Command) => void;
}

export class Editor extends React.Component<EditorProps> {
  modalBody = "";
  monaco?: Monaco;
  editor?: monaco.editor.IStandaloneCodeEditor;
  provider?: monaco.IDisposable;

  editorDidMount = (
    editor: monaco.editor.IStandaloneCodeEditor,
    monaco: Monaco
  ) => {
    this.monaco = monaco;
    this.editor = editor;
    editor.focus();
    this.provider?.dispose();
    this.provider = this.monaco?.languages.registerCodeLensProvider("rust", {
      provideCodeLenses: () => {
        return this.getInvokes();
      },
      resolveCodeLens: (model: any, codeLens: any, token: any) => {
        return codeLens;
      },
    });
  };

  addCommandCb = (
    method: "deploy" | "invoke" | "subscription",
    params: Record<string, any>
  ) => {
    return this.editor?.addCommand(
      0,
      () => {
        this.props.onEvent({
          type: method,
          params,
        });
      },
      method
    );
  };

  getInvokes = () => {
    const code = this.props.manager.getFileContent(this.props.fileId) || "";
    const lenses = [];
    const contractIdx = findContractLineNumber(code);
    if (contractIdx !== -1) {
      const command = {
        id: this.addCommandCb("deploy", {}),
        title: "🚀 Deploy",
      };
      lenses.push({
        range: {
          startLineNumber: contractIdx,
          startColumn: 1,
        },
        id: "deploy",
        command,
      });
    }

    let impls = extractContractImplMethods(code);

    for (const impl of impls) {
      const range = {
        startLineNumber: impl.lineNumber,
        startColumn: 1,
        endLineNumber: impl.lineNumber + 1,
        endColumn: 1,
      };
      const command = {
        id: this.addCommandCb("invoke", impl),
        title: "▶ Invoke",
      };
      lenses.push({
        range,
        id: "invoke",
        command,
      });
    }

    const subs = findContractEvents(code);
    for (const sub of subs) {
      const command = {
        id: this.addCommandCb("subscription", sub as Record<string, any>),
        title: sub.event,
      };
      lenses.push({
        range: {
          startLineNumber: sub.lineNumber,
          startColumn: 1,
        },
        id: "subscribe",
        command,
      });
    }

    return {
      lenses,
      dispose: () => this.provider?.dispose(),
    };
  };

  editorCodeChange = (val: string | undefined, ev: any) => {
    if (val !== undefined) {
      this.props.manager.updateFileContent(this.props.fileId, val);
      this.provider?.dispose();
      this.provider = this.monaco?.languages.registerCodeLensProvider("rust", {
        provideCodeLenses: (model, token) => {
          return this.getInvokes();
        },
        resolveCodeLens: (model: any, codeLens: any, token: any) => {
          return codeLens;
        },
      });
    }
  };

  getLanguage() {
    const fileName = this.props.manager.getFileName(this.props.fileId);
    const idx = fileName?.lastIndexOf(".");
    if (idx !== undefined && idx >= 0) {
      const fileExt = fileName?.substring(idx + 1);
      if (fileExt === "rs") {
        return "rust";
      }
      return fileExt;
    }
  }

  render() {
    const config = this.props.config;
    const options: monaco.editor.IStandaloneEditorConstructionOptions = {
      selectOnLineNumbers: false,
      readOnly: !config.editable,
      scrollbar: {
        vertical: "hidden",
      },
      minimap: {
        enabled: config.multiFile,
      },
      lineDecorationsWidth: 0,
      lineNumbersMinChars: 0,
      lineNumbers: config.multiFile ? "on" : "off",
      folding: config.multiFile,
      contextmenu: config.multiFile,
      scrollBeyondLastLine: false 
    };
    const code = this.props.manager.getFileContent(this.props.fileId) || "";
    return (
      <MonacoEditor
        width="100%"
        height="100%"
        language={this.getLanguage()}
        theme="vs-light"
        value={code}
        options={options}
        onMount={this.editorDidMount}
        onChange={this.editorCodeChange}
      />
    );
  }
}
