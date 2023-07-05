import './style.css'
import './editor'
import { setupMonaco } from './editor'

const app = document.querySelector<HTMLDivElement>('#app')
if (app != null) {
  app.innerHTML = `
  <div style="display:flex;flex-direction:column">
    <div class="card">
      <button id="build" type="button">Build</button>
    </div>
    <div id="editor"></div>
  </div>

`
}

const editor = document.querySelector<HTMLDivElement>('#editor')
if (editor != null) {
  setupMonaco(editor, document.querySelector<HTMLButtonElement>('#build')!, document.querySelector<HTMLDivElement>('.card')!)
}
