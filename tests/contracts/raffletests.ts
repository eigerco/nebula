import { BaseTests } from './basetests'
import type { AppConfig } from '../appconfig'

export class RaffleTests extends BaseTests {
  private tokenId: string = ''

  constructor(protected config: AppConfig) {
    super(config)
  }

  async getTokenId() {
    await this.runSoroban([
      'lab',
      'token',
      'id',
      '--asset',
      'native',
      '--network',
      this.config.network
    ]).then(async data => {
      this.tokenId = data.toString().trim()
    })
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
      '--token',
      this.tokenId,
      '--max_winners_count',
      '1',
      '--ticket_price',
      '5001'
    ])
  }

  async invokeBuyTickets() {
    return await Array.from(this.accounts.keys())
      .filter(r => r !== 'admin')
      .reduce(async (p, acc) => {
        return await p.then(async r => {
          this.handleResult(r)
          return await this.invokeBuyOneTicket(acc)
        })
      }, Promise.resolve())
  }

  async invokeBuyOneTicket(acc: string) {
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
      'buy_ticket',
      '--by',
      this.accounts.get(acc)[0]
    ])
  }

  async invokePlayRaffle() {
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
      'play_raffle',
      '--random_seed',
      '1234'
    ])
  }

  async checkBalance() {
    return await Array.from(this.accounts.keys())
      .filter(r => r !== 'admin')
      .reduce(async (p, acc) => {
        return await p.then(async r => {
          this.handleResult(r)
          return await this.checkBalanceForAccount(acc)
        })
      }, Promise.resolve())
  }

  async checkBalanceForAccount(acc: string) {
    return await this.runSoroban([
      'contract',
      'invoke',
      '--id',
      this.tokenId,
      '--network',
      this.config.network,
      '--',
      'balance',
      '--id',
      this.accounts.get(acc)[0]
    ])
  }

  public async run() {
    return await this.createAccounts()
      .then(async r => {
        this.handleResult(r)
        await this.deployContract('raffle')
      })
      .then(async r => {
        this.handleResult(r)
        await this.getTokenId()
      })
      .then(async r => {
        this.handleResult(r)
        return await this.invokeInit()
      })
      .then(async r => {
        this.handleResult(r)
        return await this.invokeBuyTickets()
      })
      .then(async r => {
        this.handleResult(r)
        return await this.invokePlayRaffle()
      })
      .then(async r => {
        this.handleResult(r)
        return await this.checkBalance()
      })
  }
}
