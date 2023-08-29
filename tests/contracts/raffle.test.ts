import { BaseTest } from './basetest'
import { AppConfig } from '../appconfig'
import test from 'node:test'
import assert from 'assert/strict'

export class RaffleTest extends BaseTest {
  private tokenId: string = ''

  constructor(protected config: AppConfig) {
    super(config)
  }

  setupAccountNames() {
    this.accounts.set('admin', [])
    this.accounts.set('player_1', [])
  }

  async getTokenId() {
    // prettier-ignore
    await this.runSoroban(['lab', 'token',
      'id', '--asset',
      'native', '--network',
      this.config.network
    ]).then(async data => {
      this.tokenId = data.toString().trim()
    })
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
        '--token', this.tokenId,
        '--max_winners_count', '1',
        '--ticket_price', '2'
    ])
  }

  async invokeBuyTickets() {
    return await Array.from(this.accounts.keys())
      .filter(r => r !== 'admin')
      .reduce(async (p, acc) => {
        return await p.then(async r => {
          this.printLogs(r)
          return await this.invokeBuyOneTicket(acc)
        })
      }, Promise.resolve())
  }

  async invokeBuyOneTicket(acc: string) {
    // prettier-ignore
    return await this.runSoroban(['contract', 'invoke',
      '--id', this.contractId,
      '--source', acc,
      '--network', this.config.network,
      '--',
      'buy_ticket',
        '--by', this.accounts.get(acc)[0]
    ])
  }

  async invokePlayRaffle() {
    // prettier-ignore
    return await this.runSoroban(['contract', 'invoke',
      '--id', this.contractId,
      '--source', 'admin',
      '--network', this.config.network,
      '--',
      'play_raffle',
        '--random_seed', '1234'
    ])
  }

  async checkBalance() {
    return await Array.from(this.accounts.keys())
      .filter(r => r !== 'admin')
      .reduce(async (p, acc) => {
        return await p.then(async r => {
          this.printLogs(r)
          return await this.checkBalanceForAccount(acc)
        })
      }, Promise.resolve())
  }

  async checkBalanceForAccount(acc: string) {
    // prettier-ignore
    return await this.runSoroban(['contract', 'invoke',
      '--id', this.tokenId,
      '--network', this.config.network,
      '--',
      'balance',
        '--id', this.accounts.get(acc)[0]
    ])
  }
}

void test('Raffle Contract', async t => {
  const config = new AppConfig()
  const raffle = new RaffleTest(config)
  await t.test('createAccounts', async t => {
    await raffle.createAccounts().then((r) => { raffle.printLogs(r) })
  })
  await t.test('deployContract', async t => {
    await raffle.deployContract('raffle').then((r) => { raffle.printLogs(r) })
  })
  await t.test('getTokenId', async t => {
    await raffle.getTokenId().then((r) => { raffle.printLogs(r) })
  })
  await t.test('invokeInit', async t => {
    await raffle.invokeInit().then((r) => { raffle.printLogs(r) })
  })
  await t.test('invokeBuyTickets', async t => {
    await raffle.invokeBuyTickets().then((r) => { raffle.printLogs(r) })
  })
  await t.test('invokePlayRaffle', async t => {
    await raffle.invokePlayRaffle().then((r) => { raffle.printLogs(r) })
  })
  await t.test('checkBalance', async t => {
    await raffle.checkBalance().then((r) => { raffle.printLogs(r) })
  })
})
