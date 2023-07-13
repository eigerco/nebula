import React from 'react'
import Highlight from 'react-highlight'
import 'highlight.js/styles/github-dark.css'

export function Editor ({
  contractTrait,
  codeGen,
  contractName,
  author,
  license
}: any) {
  codeGen.generateHeader(author, license)
  codeGen.generateContractCode(contractTrait, contractName)
  const contractCode = codeGen.getCode()

  return (
    <div className="Editor">
      <Highlight className="language-rust">{contractCode}</Highlight>
    </div>
  )
}
