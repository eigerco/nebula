import hljs from 'highlight.js'
import 'highlight.js/styles/github-dark.css'
import { CodeGen } from './codegen'
import './style.css'

const codeGen = new CodeGen()
let contractCode = codeGen.generateHeader('author', 'MIT')
contractCode += '\n' + codeGen.generateImports(['Lottery'])
contractCode += '\n' + codeGen.generateContract('MyContract', ['true', '100', '10', '1'])

const html = hljs.highlight(contractCode, { language: 'rust' }).value

const editor = document.querySelector<HTMLDivElement>('#editor')
if (editor != null) {
  editor.innerHTML = '<pre><code class="language-rust">' + html + '</code></pre>'
}
