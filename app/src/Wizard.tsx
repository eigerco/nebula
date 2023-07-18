import React, { useState } from 'react'
import Button from 'react-bootstrap/Button'
import Modal from 'react-bootstrap/Modal'
import { CodeGen } from './codegen/codegen'
import { Editor } from './Editor'
import { Toolbox } from './Toolbox'
import { InvokeCommandGen } from './codegen/invokecommandgen'
import { Navbar } from './Navbar'
import './Wizard.css'

export function Wizard() {
  const [contractTrait, setContractTrait] = useState('Lottery')
  const [contractName, setContractName] = useState('MyContract')
  const [contractParams, setContractParams] = useState([])
  const [author, setAuthor] = useState('')
  const [license, setLicense] = useState('')
  const [showInvokeModal, setShowInvokeModal] = useState(false)
  const [modalTitle, setModalTitle] = useState('')
  const [modalBody, setModalBody] = useState('')
  const [modalBackground, setModalBackground] = useState('console')

  const codeGen = new CodeGen()

  function handleClick(type: string) {
    setModalBackground('normal')
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
      setModalBackground('console')
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

  function handleCopyToClipboard() {
    const invokeGen = new InvokeCommandGen()
    const invokeCode = invokeGen.generateInvokeCommand(
      contractTrait,
      contractName,
      contractParams
    )
    void navigator.clipboard.writeText(invokeCode)
  }

  return (
    <div className="Wizard">
      <Navbar currentPage="#/" />
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
              <pre className={modalBackground}>
                <code>{modalBody}</code>
              </pre>
            </Modal.Body>
            <Modal.Footer>
              {modalBackground === 'console' && (
                <Button variant="secondary" onClick={handleCopyToClipboard}>
                  <i className="bi bi-clipboard"></i>
                  Copy to clipboard
                </Button>
              )}
              <Button variant="secondary" onClick={handleInvokeModalClose}>
                <i className="bi bi-x-circle"></i>
                Close
              </Button>
            </Modal.Footer>
          </Modal>
        </div>
      </div>
    </div>
  )
}
