import React from 'react'
import Form from 'react-bootstrap/Form'
import { ContractParams } from './contractparams/ContractParams'
import './Toolbox.css'
import logo from './logo.png'

export function Toolbox({ contractName, onContractNameChanged, contractType, onContractTypeChanged, updateParams, author, onAuthorChanged, license, onLicenseChanged, 
                          handleClick }: any) {  
  return (
    <div className='Toolbox'>
        <img src={logo} className="logo" alt="logo"/>
        <div className="input-group mb-2">
            <span className="input-group-text">Name</span>
            <input type="text" className="form-control" placeholder="Contract name" aria-label="Name" aria-describedby="basic-addon1" value={contractName}
              onChange={e => onContractNameChanged(e.target.value)}/>
        </div>
        <div className="input-group mb-2">
          <span className="input-group-text">Contract trait</span>
          <Form.Select className="p-2" onChange={e => onContractTypeChanged(e.target.value)}>
            <option>Lottery</option>
            <option>Voting</option>
          </Form.Select>
        </div>
        <div className="buttons">
              <button type="button" className="btn btn-secondary" onClick={e => handleClick('Compile')}><i className="bi bi-hammer"></i>Compile</button>
              <button type="button" className="btn btn-secondary" onClick={e => handleClick('Deploy')}><i className="bi bi-cloud-upload"></i>Deploy</button>
              <button type="button" className="btn btn-secondary" onClick={e => handleClick('Invoke')}><i className="bi bi-gear"></i>Invoke</button>
        </div> 
          <div className="input-group mb-2">
          <label className="settings">
              Init params:
          </label>
          <div className="contract-settings">
              <ContractParams contractType={contractType} updateParams={updateParams}/>
          </div>
        </div>
        <div className="footer">
            <div className="input-group mb-2">
              <span className="input-group-text">Author</span>
              <input type="text" className="form-control" aria-label="License" aria-describedby="basic-addon1" value={author}
                  onChange={e => onAuthorChanged(e.target.value)}/>
            </div>
            <div className="input-group mb-2">
              <span className="input-group-text">License</span>
              <input type="text" className="form-control" aria-label="License" aria-describedby="basic-addon1" value={license}
                  onChange={e => onLicenseChanged(e.target.value)}/>
            </div>
            <div className="buttons">
                <button type="button" className="btn btn-secondary" onClick={e => handleClick('Copy')}><i className="bi bi-clipboard"></i>Copy to clipboard</button>
                <button type="button" className="btn btn-secondary" onClick={e => handleClick('Download')}><i className="bi bi-cloud-arrow-down"></i>Download</button>
                <button type="button" className="btn btn-secondary" onClick={e => handleClick('Open')}><i className="bi bi-folder2-open"></i>Open in Pulsar</button>
            </div>
        </div>
  </div>
  );
}
