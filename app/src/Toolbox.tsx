import React from 'react'
import Form from 'react-bootstrap/Form'
// import './Toolbox.css'

export function Toolbox({
  contractName,
  onContractNameChanged,
  onContractTraitChanged,
  author,
  onAuthorChanged,
  license,
  onLicenseChanged,
  handleClick,
}: any) {
  return (
    <div className="Toolbox">
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
        <span className="input-group-text">Interface</span>
        <Form.Select
          className="p-2"
          onChange={e => onContractTraitChanged(e.target.value)}
        >
          <option>Raffle</option>
          <option>Voting</option>
          <option disabled>Auction</option>
          <option disabled>MarketPlace</option>
          <option disabled>FundMe</option>
          <option disabled>Math</option>
          <option disabled>PaymentSplit</option>
          {/* <option disabled>NFT</option> */}
        </Form.Select>
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
      <div className="buttons">
        <div className="row mt-1">
          <div className="col">
            <button
              type="button"
              className="btn btn-secondary w-100"
              onClick={e => handleClick('Copy')}
              title="Copy to clipboard"
            >
              <i className="bi bi-clipboard"></i>Copy
            </button>
          </div>
          <div className="col">
            <button
              type="button"
              className="btn btn-secondary w-100"
              onClick={e => handleClick('Open')}
              title="Open in Playground"
            >
              <i className="bi bi-code-square"></i> Open in Playground
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}
