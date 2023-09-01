import React, { useState } from 'react'
import Button from 'react-bootstrap/Button'
import Modal from 'react-bootstrap/Modal'
import { ContractsCodeGen } from './contractsources/contractscodegen'
import { Editor } from './Editor'
import { Toolbox } from './Toolbox'

export default function Wizard() {
  const [contractTrait, setContractTrait] = useState('Raffle')
  const [contractName, setContractName] = useState('MyContract')
  const [author, setAuthor] = useState('eigerco')
  const [license, setLicense] = useState('MIT')
  const [showInvokeModal, setShowInvokeModal] = useState(false)
  const [modalTitle, setModalTitle] = useState('')
  const [modalBody, setModalBody] = useState('')
  const [modalBackground, setModalBackground] = useState('console')

  const codeGen = new ContractsCodeGen()

  function handleClick(type: string) {
    setModalBackground('normal')
    // TODO: Replace these with useReducer
    // https://react.dev/reference/react/useReducer
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
      <div className="flex flex-grow-1">
        <div className="col-3 position-fixed">
          <div className="flex-column flex-nowrap py-3">
            <Toolbox
              contractName={contractName}
              onContractNameChanged={setContractName}
              onContractTraitChanged={setContractTrait}
              author={author}
              onAuthorChanged={setAuthor}
              license={license}
              onLicenseChanged={setLicense}
              handleClick={handleClick}
            />
          </div>
        </div>
        <div className="col-8 offset-4">
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
