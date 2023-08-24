import { BaseTests } from './basetests'
import type { AppConfig } from '../appconfig'

export class RaffleTests extends BaseTests {
  private tokenId: string = ''

  constructor(protected config: AppConfig) {
    super(config)
  }

  async getTokenId() {
    let resultOk = true
    await this.runSoroban([
      'lab',
      'token',
      'id',
      '--asset',
      'native',
      '--network',
      this.config.network,
    ]).then(
      data => {
        this.tokenId = data.toString().trim()
      },
      error => {
        this.logs.push(error.toString())
        resultOk = false
        console.error('error: ', error.toString())
      }
    )
    return resultOk
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
      '--token',
      this.tokenId,
      '--max_winners_count',
      '1',
      '--ticket_price',
      '5001',
    ]).then(
      data => {
        if (this.config.showLogs) {
          console.log(data.toString().trim())
        }
      },
      error => {
        this.logs.push(error.toString())
        resultOk = false
        console.error('error: ', error.toString())
      }
    )
    return resultOk
  }

  async invokeBuyTickets() {
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
          'buy_ticket',
          '--by',
          this.accounts.get(acc)[0],
        ]).then(
          data => {
            if (this.config.showLogs) {
              console.log(data.toString().trim())
            }
          },
          error => {
            this.logs.push(error.toString())
            resultOk = false
            console.error('error: ', error.toString())
          }
        )
      }
    }
    return resultOk
  }

  async invokePlayRaffle() {
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
      'play_raffle',
      '--random_seed',
      '1234',
    ]).then(
      data => {
        if (this.config.showLogs) {
          console.log(data.toString().trim())
        }
      },
      error => {
        this.logs.push(error.toString())
        resultOk = false
        console.error('error: ', error.toString())
      }
    )
    return resultOk
  }

  async checkBalance() {
    let resultOk = true
    for (const acc of this.accounts.keys()) {
      if (acc !== 'admin') {
        await this.runSoroban([
          'contract',
          'invoke',
          '--id',
          this.tokenId,
          '--network',
          this.config.network,
          '--',
          'balance',
          '--id',
          this.accounts.get(acc)[0],
        ]).then(
          data => {
            if (this.config.showLogs) {
              console.log(data.toString().trim())
            }
          },
          error => {
            this.logs.push(error.toString())
            resultOk = false
            console.error('error: ', error.toString())
          }
        )
      }
    }
    return resultOk
  }

  public async run(): Promise<boolean> {
    let result = await this.createAccounts()
    if (!result) return false
    result = await this.deployContract('raffle')
    if (!result) return false
    result = await this.getTokenId()
    if (!result) return false
    result = await this.invokeInit()
    if (!result) return false
    result = await this.invokeBuyTickets()
    if (!result) return false
    result = await this.invokePlayRaffle()
    if (!result) return false
    return await this.checkBalance()
  }
}
