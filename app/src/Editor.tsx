import React from 'react'
import Button from 'react-bootstrap/Button'
import Modal from 'react-bootstrap/Modal'
import MonacoEditor from '@monaco-editor/react'

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
  monaco: any
  editor: any

  editorDidMount = (ed: any, mon: any) => {
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
        return outerThis.props.codeGen.getInvokes(outerThis.props.contractTrait, commandId)
      },
      resolveCodeLens: function (model: any, codeLens: any, token: any) {
        return codeLens
      }
    })
  }

  private handleInvokeModalClose(outerThis: any) {
    console.log(this)
    outerThis.setState({ showInvokeModal: false })
  }

  render() {
    const options = {
      selectOnLineNumbers: true,
      readOnly: true
    }
    this.props.codeGen.generateHeader(this.props.author, this.props.license)
    this.props.codeGen.generateContractCode(this.props.contractTrait, this.props.contractName)
    const contractCode = this.props.codeGen.getCode()

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
            <pre>
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
        value={contractCode}
        options={options}
        onMount={this.editorDidMount}
      />
    </div>
    )
  }
}
