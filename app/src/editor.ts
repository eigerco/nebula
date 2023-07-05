import hljs from "highlight.js";
import 'highlight.js/styles/github-dark.css';
import { CodeGen } from "./codegen";
import "./style.css";


let codeGen = new CodeGen();
let contractCode = codeGen.generateHeader('author', 'MIT');
contractCode += '\n' + codeGen.generateImports(['SomeContract', 'OtherContract']);
contractCode += '\n' + codeGen.generateContract('MyContract', true);

let html = hljs.highlight(contractCode, {language: 'rust'}).value

document.querySelector<HTMLDivElement>("#editor")!.innerHTML = 
  '<pre><code class="language-rust">' + html + '</code></pre>';
