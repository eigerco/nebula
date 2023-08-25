import { BaseTests } from './basetests'
import { AppConfig } from '../appconfig'
import test from 'node:test'
import assert from 'assert/strict'

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
      this.config.network
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
      '5001'
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
          this.accounts.get(acc)[0]
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
      '1234'
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
          this.accounts.get(acc)[0]
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
}

void test('Raffle Contract', async t => {
  const config = new AppConfig()
  config.parseConfig()
  const raffle = new RaffleTests(config)
  await t.test('createAccounts', async t => {
    await raffle.createAccounts()
  })

  await t.test('deployContract', async t => {
    await raffle.deployContract('raffle')
  })
  await t.test('getTokenId', async t => {
    await raffle.getTokenId()
  })
  await t.test('invokeInit', async t => {
    await raffle.invokeInit()
  })
  await t.test('invokeBuyTickets', async t => {
    await raffle.invokeBuyTickets()
  })
  await t.test('invokePlayRaffle', async t => {
    await raffle.invokePlayRaffle()
  })
  await t.test('checkBalance', async t => {
    await raffle.invokePlayRaffle()
  })
})
