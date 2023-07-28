import React from 'react'

interface Props {
  updateParams: any
}

// TODO: Use functional component
export class RaffleParams extends React.Component<Props> {
  state = {
    adminAccount: '',
    token: '',
    winners: 1,
    ticketPrice: 1,
  }

  constructor(props: any) {
    super(props)
    this.props.updateParams([
      this.state.adminAccount,
      this.state.token,
      this.state.winners,
      this.state.ticketPrice,
    ])
  }

  render() {
    return (
      <form>
        <div className="RaffleParams">
          <div className="input-group mb-2">
            <span className="input-group-text">Admin account</span>
            <input
              type="text"
              className="form-control"
              aria-label="Admin account"
              aria-describedby="basic-addon1"
              value={this.state.adminAccount}
              onChange={e => {
                this.setState({ adminAccount: e.target.value })
                this.props.updateParams([
                  e.target.value,
                  this.state.token,
                  this.state.winners,
                  this.state.ticketPrice,
                ])
              }}
            />
          </div>
          <div className="input-group mb-2">
            <span className="input-group-text">Token address</span>
            <input
              type="text"
              className="form-control"
              aria-label="Token address"
              aria-describedby="basic-addon1"
              value={this.state.token}
              onChange={e => {
                this.setState({ token: e.target.value })
                this.props.updateParams([
                  this.state.adminAccount,
                  e.target.value,
                  this.state.winners,
                  this.state.ticketPrice,
                ])
              }}
            />
          </div>
          <div className="input-group mb-2">
            <span className="input-group-text">Max winners</span>
            <input
              type="number"
              min="1"
              max="1000"
              className="form-control"
              aria-label="Max number of winners"
              aria-describedby="basic-addon1"
              value={this.state.winners}
              onChange={e => {
                this.setState({ winners: +e.target.value })
                this.props.updateParams([
                  this.state.adminAccount,
                  this.state.token,
                  e.target.value,
                  this.state.ticketPrice,
                ])
              }}
            />
          </div>
          <div className="input-group mb-2">
            <span className="input-group-text">Ticket price</span>
            <input
              type="number"
              min="1"
              max="1000"
              className="form-control"
              aria-label="Ticket price"
              aria-describedby="basic-addon1"
              value={this.state.ticketPrice}
              onChange={e => {
                this.setState({ ticketPrice: +e.target.value })
                this.props.updateParams([
                  this.state.adminAccount,
                  this.state.token,
                  this.state.winners,
                  e.target.value,
                ])
              }}
            />
          </div>
        </div>
      </form>
    )
  }
}
