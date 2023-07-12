import React from 'react'
import Highlight from 'react-highlight'
import 'highlight.js/styles/github-dark.css'

export function Editor ({
  contractType,
  codeGen,
  contractName,
  params,
  author,
  license
}: any) {
  codeGen.setHeader(author, license)
  codeGen.setContractCode(contractType, contractName, params)
  const contractCode = codeGen.generateCode()

  return (
    <div className="Editor">
      <Highlight className="language-rust">{contractCode}</Highlight>
    </div>
  )
}
