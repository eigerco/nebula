import StellarSdk from 'stellar-sdk'
import SorobanClient from 'soroban-client'
import { BaseTests } from './basetests'
import type { AppConfig } from '../appconfig'

export class VotingTests extends BaseTests {
  constructor(protected config: AppConfig) {
    super(config)
  }

  async invokeInit() {
    let resultOk = true
    await this.runSoroban([
      'contract',
      'invoke',
      '--id',
      this.contractId,
      '--source',
      'admin',
      '--network',
      this.config.network,
      '--',
      'init',
      '--admin',
      this.accounts.get('admin')[0],
      '--voting_period_secs',
      '3600',
      '--target_approval_rate_bps',
      '50000',
      '--total_voters',
      '3',
    ]).then(
      data => {
        if (this.config.showLogs) {
          console.log(data.toString().trim())
        }
      },
      error => {
        this.logs.push(error.toString())
        resultOk = false
        console.error(error)
      }
    )
    return resultOk
  }

  async invokeCreateProposal() {
    let resultOk = true
    await this.runSoroban([
      'contract',
      'invoke',
      '--id',
      this.contractId,
      '--source',
      'admin',
      '--network',
      this.config.network,
      '--',
      'create_proposal',
      '--id',
      '1',
    ]).then(
      data => {
        if (this.config.showLogs) {
          console.log(data.toString().trim())
        }
      },
      error => {
        this.logs.push(error.toString())
        resultOk = false
        console.error(error)
      }
    )
    return resultOk
  }

  async invokeVote() {
    let resultOk = true
    for (const acc of this.accounts.keys()) {
      if (acc !== 'admin') {
        await this.runSoroban([
          'contract',
          'invoke',
          '--id',
          this.contractId,
          '--source',
          acc,
          '--network',
          this.config.network,
          '--',
          'vote',
          '--voter',
          this.accounts.get(acc)[0],
          '--id',
          '1',
        ]).then(
          data => {
            if (this.config.showLogs) {
              console.log(data.toString().trim())
            }
          },
          error => {
            this.logs.push(error.toString())
            resultOk = false
            console.error(error)
          }
        )
      }
    }
    return resultOk
  }

  async checkResults() {
    const server = new StellarSdk.Server(this.config.server)
    let resultOk = false
    await server
      .transactions()
      .forAccount(this.accounts.get('player_2')[0])
      .call()
      .then(r => {
        const tx = r.records[r.records.length - 1]
        // use SorobanClient here, as StellarSdk.xdr has problems with parsing the meta
        const resultMeta = SorobanClient.xdr.TransactionMeta.fromXDR(
          tx.result_meta_xdr,
          'base64'
        )
        const result =
          resultMeta._value._attributes.sorobanMeta._attributes.events[0]
            ._attributes.body._value._attributes.data._value
        if (result === 6666) {
          resultOk = true
        }
      })
    return resultOk
  }

  async calculateFees() {
    await this.calculateFeesForAccount('admin')
    await this.calculateFeesForAccount('player_1')
  }

  public async run(): Promise<boolean> {
    let result = await this.createAccounts()
    if (!result) return false
    result = await this.deployContract('voting')
    if (!result) return false
    result = await this.invokeInit()
    if (!result) return false
    result = await this.invokeCreateProposal()
    if (!result) return false
    result = await this.invokeVote()
    if (!result) return false
    await this.calculateFees()
    return await this.checkResults()
  }
}
