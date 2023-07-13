import React, { useState } from 'react'
import Button from 'react-bootstrap/Button'
import Modal from 'react-bootstrap/Modal'
import { CodeGen } from './codegen/codegen'
import { Editor } from './Editor'
import { Toolbox } from './Toolbox'
import { InvokeCommandGen } from './codegen/invokecommandgen'
import { Navbar } from './Navbar'

export function Wizard() {
  const [contractTrait, setContractTrait] = useState('Lottery')
  const [contractName, setContractName] = useState('MyContract')
  const [contractParams, setContractParams] = useState([])
  const [author, setAuthor] = useState('')
  const [license, setLicense] = useState('')
  const [showInvokeModal, setShowInvokeModal] = useState(false)
  const [modalTitle, setModalTitle] = useState('')
  const [modalBody, setModalBody] = useState('')

  const codeGen = new CodeGen()

  function handleClick(type: string) {
    if (type === 'Download') {
      setModalTitle('Download')
      setModalBody('Not implemented yet')
      setShowInvokeModal(true)
    }
    if (type === 'Copy') {
      const code = codeGen.getCode()
      void navigator.clipboard.writeText(code)
    }
    if (type === 'Deploy') {
      setModalTitle('Deploy')
      setModalBody('Not implemented yet')
      setShowInvokeModal(true)
    }
    if (type === 'Invoke') {
      const invokeGen = new InvokeCommandGen()
      setModalTitle('Invoke contract')
      setModalBody(
        invokeGen.generateInvokeCommand(
          contractTrait,
          contractName,
          contractParams
        )
      )
      setShowInvokeModal(true)
    }
    if (type === 'Open') {
      setModalTitle('Open in Pulsar')
      setModalBody('Not implemented yet')
      setShowInvokeModal(true)
    }
    if (type === 'Compile') {
      setModalTitle('Compile')
      setModalBody('Not implemented yet')
      setShowInvokeModal(true)
    }
  }
  const handleInvokeModalClose = () => {
    setShowInvokeModal(false)
  }

  return (
    <div className="Wizard">
      <Navbar currentPage="/wizard" />
      <div className="row flex-grow-1">
        <div className="col-3">
          <Toolbox
            contractName={contractName}
            onContractNameChanged={setContractName}
            contractTrait={contractTrait}
            onContractTraitChanged={setContractTrait}
            author={author}
            onAuthorChanged={setAuthor}
            license={license}
            onLicenseChanged={setLicense}
            updateParams={setContractParams}
            handleClick={handleClick}
          />
        </div>
        <div className="col-8">
          <Editor
            contractTrait={contractTrait}
            contractName={contractName}
            author={author}
            license={license}
            codeGen={codeGen}
          />
        </div>
        <div
          className="modal show"
          style={{ display: 'block', position: 'initial' }}
        >
          <Modal show={showInvokeModal} onHide={handleInvokeModalClose}>
            <Modal.Header closeButton>
              <Modal.Title>{modalTitle}</Modal.Title>
            </Modal.Header>
            <Modal.Body>
              <p>
                <pre>
                  <code>{modalBody}</code>
                </pre>
              </p>
            </Modal.Body>
            <Modal.Footer>
              <Button variant="secondary" onClick={handleInvokeModalClose}>
                Close
              </Button>
            </Modal.Footer>
          </Modal>
        </div>
      </div>
    </div>
  )
}
