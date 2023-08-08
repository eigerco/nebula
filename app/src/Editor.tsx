import React from 'react'
import Button from 'react-bootstrap/Button'
import Modal from 'react-bootstrap/Modal'
import MonacoEditor from '@monaco-editor/react'
import './Editor.css'
import { ContractService } from './contractsources/contractservice'

interface Props {
  contractTrait: any
  codeGen: any
  contractName: any
  author: any
  license: any
}

export class Editor extends React.Component<Props> {
  state = {
    showInvokeModal: false
  }

  modalBody = ''
  contractCode = ''
  monaco: any
  editor: any

  contractService = new ContractService()

  editorDidMount = (ed: any, mon: any) => {
    console.log('reget')
    this.contractService.readContracts().then(() => {
      // force redraw editor
      this.forceUpdate()
    }).catch((e) => { console.error(e) })

    this.monaco = mon
    this.editor = ed
    ed.focus()
    this.updateInvokes(this)
  }

  private updateInvokes(outerThis: any) {
    const commandId = (method: string, params: string) => {
      return this.editor.addCommand(
        0,
        function () {
          const contractName: string = outerThis.props.contractName
          // services available in `ctx`
          outerThis.modalBody = `soroban contract invoke \\
    --wasm ${contractName}.wasm \\
    --id 1 \\
    -- \\
    ${method} \\
        ${params}`
          outerThis.setState({ showInvokeModal: true })
        },
        ''
      )
    }
    this.monaco.languages.registerCodeLensProvider('rust', {
      provideCodeLenses: function (model: any, token: any) {
        return outerThis.props.codeGen.getInvokes(commandId)
      },
      resolveCodeLens: function (model: any, codeLens: any, token: any) {
        return codeLens
      }
    })
  }

  private handleInvokeModalClose(outerThis: any) {
    outerThis.setState({ showInvokeModal: false })
  }

  generateContractCode() {
    this.props.codeGen.generateHeader(this.props.author, this.props.license)
    const traitLowerCase: string = this.props.contractTrait.toLowerCase()
    const originalCode = this.contractService.getContractsContent(traitLowerCase)
    this.props.codeGen.generateContractCode(originalCode, this.props.contractName)
    this.contractCode = this.props.codeGen.getCode()
  }

  render() {
    const options = {
      selectOnLineNumbers: true,
      readOnly: true
    }
    this.generateContractCode()

    return (
    <div className="Editor">
      <div
        className="modal show"
        style={{ display: 'block', position: 'initial' }}
      >
        <Modal show={this.state.showInvokeModal} onHide={() => { this.handleInvokeModalClose(this) } }>
          <Modal.Header closeButton>
            <Modal.Title>Invoke command example</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <pre className='console'>
              <code>{this.modalBody}</code>
            </pre>
          </Modal.Body>
          <Modal.Footer>
            <Button variant="secondary" onClick={() => { this.handleInvokeModalClose(this) } }>
              <i className="bi bi-x-circle"></i>
              Close
            </Button>
          </Modal.Footer>
        </Modal>
      </div>
      <MonacoEditor
        width="100%"
        height="90vh"
        language="rust"
        theme="vs-dark"
        value={this.contractCode}
        options={options}
        onMount={this.editorDidMount}
      />
    </div>
    )
  }
}
