import * as monaco from 'monaco-editor'
import { type Environment } from 'monaco-editor/esm/vs/editor/editor.api'
import 'monaco-editor/esm/vs/basic-languages/rust/rust.contribution.js'

declare global {
  interface Window {
    MonacoEnvironment?: Environment | undefined
  }
}

window.MonacoEnvironment = {
  getWorker(_workerId, label) {
    switch (label) {
      case 'css':
        return new Worker(
          new URL(
            'monaco-editor/min/vs/language/css/css.worker',
            import.meta.url
          )
        )
      case 'editorWorkerService':
        return new Worker(
          new URL('monaco-editor/min/vs/editor/editor.worker', import.meta.url)
        )
      case 'json':
        return new Worker(
          new URL(
            'monaco-editor/min/vs/language/json/json.worker',
            import.meta.url
          )
        )
      case 'yaml':
        return new Worker(new URL('monaco-yaml/yaml.worker', import.meta.url))
      default:
        throw new Error(`Unknown label ${label}`)
    }
  },
}

export function setupMonaco(
  element: HTMLElement,
  build: HTMLButtonElement,
  download: HTMLButtonElement
): monaco.editor.IStandaloneCodeEditor {
  const editor = monaco.editor.create(element, {
    language: 'rust',
    lineNumbers: 'on',
    wordWrap: 'on',
    scrollBeyondLastLine: false,
    glyphMargin: true,
    value: `#![no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec};

pub struct Contract;
    
#[contractimpl]
impl Contract {
    pub fn hello(env: Env, receiver: Symbol) -> Vec<Symbol> {
        vec![&env, Symbol::short("Hello"), receiver]
    }
}`,
  })

  build.addEventListener('click', event => {
    const code =  editor.getValue();
    let utf8Encode = new TextEncoder();
    fetch("http://0.0.0.0:4000/run", {
      method: "POST",
      body: utf8Encode.encode(code)
    }).then(console.log)
  })

  return editor
}
