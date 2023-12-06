import React from "react";
import MonacoEditor, { Monaco } from "@monaco-editor/react";
import { ProjectManager } from "../../project-manager";
import * as monaco from "monaco-editor";

interface Action {}

export interface EditorProps {
  editable: boolean;
  actions: Action[];
  fileId: number;
  manager: ProjectManager;
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
    this.provider?.dispose()
    this.monaco = monaco;
    this.editor = editor;
    editor.focus();
    const commandId = (method: string, params: string) => {
      return this.editor?.addCommand(
        0,
        function () {
          console.log(method, params);
        },
        method
      );
    };
    this.provider = this.monaco?.languages.registerCodeLensProvider("rust", {
      provideCodeLenses: () => {
        return this.getInvokes(commandId);
      },
      resolveCodeLens: (model: any, codeLens: any, token: any) => {
        return codeLens;
      },
    });
  };

  getInvokes = (commandId: any) => {
    const code = this.props.manager.getFileContent(this.props.fileId) || "";
    const lenses = [];
    const contractIdx = findContractLineNumber(code);
    if (contractIdx !== -1) {
      const command = {
        id: commandId("Deploy"),
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
      const funName = impl.method;
      const funParams = impl.parameters;
      const command = {
        id: commandId(funName, funParams),
        title: "▶ Invoke",
      };
      lenses.push({
        range,
        id: "invoke",
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
    const options: monaco.editor.IStandaloneEditorConstructionOptions = {
      selectOnLineNumbers: false,
      readOnly: !this.props.editable,
      scrollbar: {
        vertical: "hidden",
      },
      minimap: {
        enabled: false,
      },
      lineDecorationsWidth: 0,
      lineNumbersMinChars: 0,
      lineNumbers: "off",
      glyphMargin: true,
      folding: false,
      contextmenu: false,
    };
    const code = this.props.manager.getFileContent(this.props.fileId) || "";
    return (
      <MonacoEditor
        width="100%"
        height="100%"
        language={this.getLanguage()}
        theme="vs-dark"
        value={code}
        options={options}
        onMount={this.editorDidMount}
        onChange={this.editorCodeChange}
      />
    );
  }
}

function findContractLineNumber(code: string) {
  const lines = code.split("\n");
  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    if (line.trim().startsWith("#[contract]")) {
      return index + 1;
    }
  }
  return -1;
}

function extractContractImplMethods(rustCode: string) {
  const lines = rustCode.split("\n");
  const methods: ContractFunction[] = [];
  let insideContractImpl = false;

  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    const trimmedLine = line.trim();

    if (trimmedLine.startsWith("#[contractimpl]")) {
      insideContractImpl = true;
    } else if (insideContractImpl && trimmedLine.startsWith("pub fn")) {
      const methodSignature = trimmedLine.match(
        /pub fn (\w+)\(([^)]*)\) -> (\w+<([^>]*)>)\s*{/
      );
      if (methodSignature && methodSignature[1]) {
        const methodName = methodSignature[1];
        const parameters = methodSignature[2]
          .split(",")
          .map((param) => param.trim());
        const returnType = methodSignature[3];
        const returnTypeGenerics = methodSignature[4]
          .split(",")
          .map((type) => type.trim());

        methods.push({
          method: methodName,
          parameters: parameters.reduce(
            (prev, str) => {
              let [key, value] = str.split(":");
              return {
                ...prev,
                [key.trim()]: value.trim(),
              };
            },
            {} as Record<string, string>
          ),
          returnType: returnType,
          lineNumber: index + 1,
          returnTypeGenerics: returnTypeGenerics,
        });
      }
    } else if (insideContractImpl && line.startsWith("}")) {
      insideContractImpl = false;
      break;
    }
  }

  return methods;
}

interface ContractFunction {
  method: string;
  parameters: Record<string, string>;
  returnType: string;
  lineNumber: number;
  returnTypeGenerics: string[];
}
