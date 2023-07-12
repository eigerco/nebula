import React from 'react'

interface Props {
  updateParams: any
}

export class VotingParams extends React.Component<Props> {
  state = {
    adminAccount: '',
    votingPeriod: 3600,
    targetApprovalRateBps: 5000,
    totalVoters: 2,
  }

  constructor(props: any) {
    super(props)
    this.props.updateParams([
      this.state.adminAccount,
      this.state.votingPeriod,
      this.state.targetApprovalRateBps,
      this.state.totalVoters,
    ])
  }
  render() {
    return (
      <form>
        <div className="LotteryParams">
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
                  this.state.votingPeriod,
                  this.state.targetApprovalRateBps,
                  this.state.totalVoters,
                ])
              }}
            />
          </div>
          <div className="input-group mb-2">
            <span className="input-group-text">Voting period [s]</span>
            <input
              type="number"
              min="0"
              max="1000000"
              className="form-control"
              aria-label="Voting period"
              aria-describedby="basic-addon1"
              value={this.state.votingPeriod}
              onChange={e => {
                this.setState({ votingPeriod: +e.target.value })
                this.props.updateParams([
                  this.state.adminAccount,
                  e.target.value,
                  this.state.targetApprovalRateBps,
                  this.state.totalVoters,
                ])
              }}
            />
          </div>
          <div className="input-group mb-2">
            <span className="input-group-text">Target approval rate</span>
            <input
              type="number"
              min="0"
              max="1000000"
              className="form-control"
              aria-label="Max number of winners"
              aria-describedby="basic-addon1"
              value={this.state.targetApprovalRateBps}
              onChange={e => {
                this.setState({ targetApprovalRateBps: +e.target.value })
                this.props.updateParams([
                  this.state.adminAccount,
                  this.state.votingPeriod,
                  e.target.value,
                  this.state.totalVoters,
                ])
              }}
            />
          </div>
          <div className="input-group mb-2">
            <span className="input-group-text">Total voters</span>
            <input
              type="number"
              min="1"
              max="1000"
              className="form-control"
              aria-label="Ticket price"
              aria-describedby="basic-addon1"
              value={this.state.totalVoters}
              onChange={e => {
                this.setState({ totalVoters: +e.target.value })
                this.props.updateParams([
                  this.state.adminAccount,
                  this.state.votingPeriod,
                  this.state.targetApprovalRateBps,
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
