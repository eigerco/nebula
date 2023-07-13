import React from 'react'
import Form from 'react-bootstrap/Form'
import { ContractParams } from './contractparams/ContractParams'
import './Toolbox.css'
import logo from './logo.png'

export function Toolbox ({
  contractName,
  onContractNameChanged,
  contractTrait,
  onContractTraitChanged,
  updateParams,
  author,
  onAuthorChanged,
  license,
  onLicenseChanged,
  handleClick
}: any) {
  return (
    <div className="Toolbox">
      <img src={logo} className="logo" alt="logo" />
      <div className="input-group mb-2">
        <span className="input-group-text">Name</span>
        <input
          type="text"
          className="form-control"
          placeholder="Contract name"
          aria-label="Name"
          aria-describedby="basic-addon1"
          value={contractName}
          onChange={e => onContractNameChanged(e.target.value)}
        />
      </div>
      <div className="input-group mb-2">
        <span className="input-group-text">Contract trait</span>
        <Form.Select
          className="p-2"
          onChange={e => onContractTraitChanged(e.target.value)}
        >
          <option>Lottery</option>
          <option>Voting</option>
        </Form.Select>
      </div>
      <div className="buttons container text-center">
        <div className="row">
          <div className="col">
            <button
              type="button"
              className="btn btn-secondary"
              onClick={e => handleClick('Compile')}
              title="Compile"
            >
              <i className="bi bi-hammer"></i>Compile
            </button>
          </div>
          <div className="col">
            <button
              type="button"
              className="btn btn-secondary"
              onClick={e => handleClick('Download')}
              title="Download"
            >
              <i className="bi bi-cloud-arrow-down"></i>Download
            </button>
          </div>
        </div>
        <div className="row">
          <div className="col">
            <button
              type="button"
              className="btn btn-secondary"
              onClick={e => handleClick('Deploy')}
              title="Deploy"
            >
              <i className="bi bi-cloud-upload"></i>Deploy
            </button>
          </div>
          <div className="col">
            <button
              type="button"
              className="btn btn-secondary"
              onClick={e => handleClick('Invoke')}
              title="Invoke"
            >
              <i className="bi bi-gear"></i>Invoke
            </button>
          </div>
        </div>
        <div className="row">
          <div className="col">
            <button
              type="button"
              className="btn btn-secondary"
              onClick={e => handleClick('Copy')}
              title="Copy to clipboard"
            >
              <i className="bi bi-clipboard"></i>Copy
            </button>
          </div>
          <div className="col">
            <button
              type="button"
              className="btn btn-secondary"
              onClick={e => handleClick('Open')}
              title="Open in Pulsar"
            >
              <i className="bi bi-folder2-open"></i>Open in Pulsar
            </button>
          </div>
        </div>
      </div>
      <label className="params">Init params:</label>
      <div className="contract-params">
        <ContractParams
          contractTrait={contractTrait}
          updateParams={updateParams}
        />
      </div>
      <div className="input-group mb-2">
        <span className="input-group-text">Author</span>
        <input
          type="text"
          className="form-control"
          aria-label="License"
          aria-describedby="basic-addon1"
          value={author}
          onChange={e => onAuthorChanged(e.target.value)}
        />
      </div>
      <div className="input-group mb-2">
        <span className="input-group-text">License</span>
        <input
          type="text"
          className="form-control"
          aria-label="License"
          aria-describedby="basic-addon1"
          value={license}
          onChange={e => onLicenseChanged(e.target.value)}
        />
      </div>
    </div>
  )
}
