import StellarSdk from 'stellar-sdk'
import SorobanClient from 'soroban-client'
import { BaseTests } from './basetests'
import type { AppConfig } from '../appconfig'

export class VotingTests extends BaseTests {
  constructor(protected config: AppConfig) {
    super(config)
  }

  async invokeInit() {
    return await this.runSoroban([
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
      '3'
    ])
  }

  async invokeCreateProposal() {
    return await this.runSoroban([
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
      '1'
    ])
  }

  async invokeVote() {
    // invokes need to be called one after another!
    return await Array.from(this.accounts.keys())
      .filter(r => r !== 'admin')
      .reduce(async (p, acc) => {
        return await p.then(async r => {
          this.handleResult(r)
          return await this.invokeSingleVote(acc)
        })
      }, Promise.resolve())
  }

  async invokeSingleVote(acc: string) {
    return await this.runSoroban([
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
      '1'
    ])
  }

  async checkResults() {
    const server = new StellarSdk.Server(this.config.server)
    return server
      .transactions()
      .forAccount(this.accounts.get('player_2')[0])
      .call()
      .then(async r => {
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
          await Promise.resolve()
        } else {
          return await Promise.reject(new Error('Voting result different than expected'))
        }
      })
  }

  async calculateFees() {
    return await this.calculateFeesForAccount('admin').then(
      async r => await this.calculateFeesForAccount('player_1')
    )
  }

  public async run() {
    return await this.createAccounts()
      .then(async r => {
        this.handleResult(r)
        await this.deployContract('voting')
      })
      .then(async r => {
        this.handleResult(r)
        return await this.invokeInit()
      })
      .then(async r => {
        this.handleResult(r)
        return await this.invokeCreateProposal()
      })
      .then(async r => {
        this.handleResult(r)
        return await this.invokeVote()
      })
      .then(async r => {
        this.handleResult(r)
        return await this.calculateFees()
      })
      .then(async r => {
        this.handleResult(r)
        return await this.checkResults()
      })
  }
}
