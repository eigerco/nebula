import React from "react";
import MonacoEditor, { Monaco } from "@monaco-editor/react";
import { ProjectManager } from "../../project-manager";
import * as monaco from 'monaco-editor';

interface Action {}

export interface EditorProps {
  editable: boolean;
  actions: Action[];
  fileId: number;
  manager: ProjectManager;
}

export class Editor extends React.Component<EditorProps> {
  state = {
    showInvokeModal: false,
  };

  modalBody = "";
  contractCode = "";
  monaco?: Monaco;
  editor?: monaco.editor.IStandaloneCodeEditor;

  editorDidMount = (editor: monaco.editor.IStandaloneCodeEditor, monaco: Monaco) => {
    this.monaco = monaco;
    this.editor = editor;
    editor.focus();
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
    const options = {
      selectOnLineNumbers: true,
      readOnly: false,
    };
    const code = this.props.manager.getFileContent(this.props.fileId);
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
