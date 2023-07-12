import React from 'react';
import { CodeGen } from './codegen/codegen'
import Highlight from 'react-highlight'
import 'highlight.js/styles/github-dark.css'
import './Editor.css';

export function Editor({contractType, codeGen, contractName, params, author, license} : any) {  
  codeGen.setHeader(author, license)
  codeGen.setImports([contractType])
  codeGen.setContractCode(contractType, contractName, params)
  let contractCode = codeGen.generateCode()

  return (
    <div className='Editor'>
      <Highlight className='language-rust'>
        {contractCode}
      </Highlight>
    </div>
  );
}