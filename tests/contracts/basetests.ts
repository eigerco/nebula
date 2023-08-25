import StellarSdk from 'stellar-sdk'
import SorobanClient from 'soroban-client'
import { spawn } from 'child_process'
import type { AppConfig } from '../appconfig'

export enum OperationType {
  UploadContract,
  CreateContract,
  InvokeMethod,
}

export class OperationFee {
  fee: number
  operationType: OperationType
  methodName: string
}

export abstract class BaseTests {
  fees: Map<string, OperationFee[]> = new Map<string, OperationFee[]>()
  deployFee: number = 0
  invokeFee: number = 0
  protected logs: string[] = new Array<string>()
  protected accounts: Map<string, string[]> = new Map<string, string[]>([
    ['admin', []],
    ['player_1', []],
    ['player_2', []],
    ['player_3', []],
  ])

  protected contractId: string = ''

  constructor(protected config: AppConfig) {}

  async createAccounts() {
    let resultOk = true
    for (const acc of this.accounts.keys()) {
      await this.runSoroban(['config', 'identity', 'generate', acc])
      await this.runSoroban(['config', 'identity', 'address', acc]).then(
        async data => {
          const id = data.toString().trim()
          await this.runSoroban(['config', 'identity', 'show', acc]).then(
            pwd => {
              this.accounts.set(acc, [id, pwd.toString().trim()])
            }
          )
          const url = `${this.config.friendbot}?addr=${id}`
          const res = await fetch(url)
          const json = await res.json()

          if (this.config.showLogs) {
            console.log(json)
          }
        },
        error => {
          resultOk = false
          const strError: string = error.toString()
          console.error(`error: ${strError}`)
        }
      )
    }
    return resultOk
  }

  async deployContract(contract: string) {
    let result = true
    await this.runSoroban([
      'contract',
      'deploy',
      '--source',
      'admin',
      '--wasm',
      `../target/wasm32-unknown-unknown/release/${contract}.wasm`,
      '--network',
      this.config.network,
    ]).then(
      data => {
        this.contractId = data.toString().trim()
      },
      error => {
        result = false
        console.error(error)
      }
    )
    return result
  }

  protected async runSoroban(args: string[]) {
    const proc = spawn('soroban', args)
    let data: string = ''
    let error: string = ''
    for await (const chunk of proc.stdout) {
      data += chunk
    }
    for await (const chunk of proc.stderr) {
      error += chunk
    }
    const exitCode = await new Promise((resolve, reject) => {
      proc.on('close', resolve)
    })

    if (exitCode || error.length > 0) {
      throw new Error(`soroban error exit ${exitCode}, ${error}`)
    }
    return data
  }

  protected findInvokeMethodsName(envelope) {
    for (const value of envelope._value._attributes.tx._attributes.operations[0]
      ._attributes.body._value._attributes.hostFunction._value) {
      const scv = value._switch.name
      if (scv === 'scvSymbol') {
        return value._value.toString()
      }
    }
    return ''
  }

  protected async calculateFeesForAccount(account: string) {
    const server = new StellarSdk.Server(this.config.server)
    await server
      .transactions()
      .forAccount(this.accounts.get(account)[0])
      .call()
      .then(r => {
        const adminFees = new Array<OperationFee>()
        for (let i = 1; i < r.records.length; ++i) {
          const tx = r.records[i]
          const envelope = SorobanClient.xdr.TransactionEnvelope.fromXDR(
            tx.envelope_xdr,
            'base64'
          )
          const type =
            envelope._value._attributes.tx._attributes.operations[0]._attributes
              .body._value._attributes.hostFunction._switch.name
          const operationFee = new OperationFee()
          operationFee.fee = Number.parseInt(tx.fee_charged)
          if (type === 'hostFunctionTypeUploadContractWasm') {
            operationFee.operationType = OperationType.UploadContract
            operationFee.methodName = 'UploadContract'
            this.deployFee += operationFee.fee
          }
          if (type === 'hostFunctionTypeCreateContract') {
            operationFee.operationType = OperationType.CreateContract
            operationFee.methodName = 'CreateContract'
            this.deployFee += operationFee.fee
          }
          if (type === 'hostFunctionTypeInvokeContract') {
            operationFee.operationType = OperationType.InvokeMethod
            operationFee.methodName = this.findInvokeMethodsName(envelope)
            this.invokeFee += operationFee.fee
          }
          adminFees.push(operationFee)
          if (this.config.showLogs) {
            console.log(
              `${operationFee.methodName} fee: ${operationFee.fee.toString()}`
            )
          }
        }
        this.fees.set(account, adminFees)
      })
  }
}
