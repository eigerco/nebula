import StellarSdk from 'stellar-sdk'
import SorobanClient from 'soroban-client'
import { BaseTest } from './basetest'
import { AppConfig } from '../appconfig'
import test from 'node:test'
import assert from 'assert/strict'

export class VotingTest extends BaseTest {
  constructor(protected config: AppConfig) {
    super(config)
  }

  async invokeInit() {
    // prettier-ignore
    return await this.runSoroban(['contract', 'invoke',
      '--id', this.contractId,
      '--source', 'admin',
      '--network', this.config.network,
      '--',
      'init',
        '--admin', this.accounts.get('admin')[0],
        '--voting_period_secs', '3600',
        '--target_approval_rate_bps', '50000',
        '--total_voters', '3'
    ])
  }

  async invokeCreateProposal() {
    // prettier-ignore
    return await this.runSoroban(['contract', 'invoke',
      '--id', this.contractId,
      '--source', 'admin',
      '--network', this.config.network,
      '--',
      'create_proposal',
        '--id', '1'
    ])
  }

  async invokeVote() {
    // invokes need to be called one after another!
    return await Array.from(this.accounts.keys())
      .filter(r => r !== 'admin')
      .reduce(async (p, acc) => {
        return await p.then(async r => {
          this.printLogs(r)
          return await this.invokeSingleVote(acc)
        })
      }, Promise.resolve())
  }

  async invokeSingleVote(acc: string) {
    // prettier-ignore
    return await this.runSoroban(['contract', 'invoke',
      '--id', this.contractId,
      '--source', acc,
      '--network', this.config.network,
      '--',
      'vote',
        '--voter', this.accounts.get(acc)[0],
        '--id', '1'
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
        
        assert(result === 6666, new Error('Voting result different than expected'))
      })
  }

  async calculateFees() {
    return await this.calculateFeesForAccount('admin').then(
      async r => await this.calculateFeesForAccount('player_1').then(
        async r => { this.displayTestCosts() }
      )
    )
  }
}

void test('Voting Contract', async t => {
  const config = new AppConfig()
  const voting = new VotingTest(config)
  await t.test('createAccounts', async t => {
    await voting.createAccounts().then((r) => { voting.printLogs(r) })
  })
  await t.test('deployContract', async t => {
    await voting.deployContract('voting').then((r) => { voting.printLogs(r) })
  })
  await t.test('invokeInit', async t => {
    await voting.invokeInit().then((r) => { voting.printLogs(r) })
  })
  await t.test('invokeCreateProposal', async t => {
    await voting.invokeCreateProposal().then((r) => { voting.printLogs(r) })
  })
  await t.test('invokeVote', async t => {
    await voting.invokeVote().then((r) => { voting.printLogs(r) })
  })
  await t.test('checkResults', async t => {
    await voting.checkResults().then((r) => { voting.printLogs(r) })
  })
  await t.test('calculateFees', async t => {
    await voting.calculateFees().then((r) => { voting.printLogs(r) })
  })
})
