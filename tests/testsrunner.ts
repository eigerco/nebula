import { AppConfig } from './appconfig'
import type { BaseTests } from './contracts/basetests'
import { RaffleTests } from './contracts/raffletests'
import { VotingTests } from './contracts/votingtests'

export class TestsRunner {
  private readonly tests: Map<string, BaseTests> = new Map<string, BaseTests>()
  private readonly config: AppConfig = new AppConfig()

  constructor() {
    this.config.parseConfig()
    this.tests.set('voting', new VotingTests(this.config))
    this.tests.set('raffle', new RaffleTests(this.config))
  }

  public async runTests(): Promise<boolean> {
    const specificTests = this.getSpecificTests()
    let testsToRun = specificTests.length
    let succeededTests = 0
    for (const testName of specificTests) {
      const test = this.tests.get(testName)
      if (test === undefined) {
        console.error(`Could not find test ${testName}`)
        --testsToRun
        continue
      }
      console.log('---------------------------------------')
      console.log(`Running ${testName} test...`)
      console.log('---------------------------------------')
      const result = await test.run()
      if (!result) {
        break
      }
      console.log('Test costs:')
      console.log('------------')
      for (const feeName of test.fees.keys()) {
        for (const operationFee of test.fees.get(feeName)) {
          console.log(`${operationFee.methodName}: ${operationFee.fee}`)
        }
      }
      console.log('---------------------------------------')
      console.log(
        `Total contract deployment fee: ${test.deployFee}, total invoke fee: ${test.invokeFee}`
      )
      succeededTests++
    }
    console.log('---------------------------------------')
    console.log(
      `Ran ${testsToRun} tests, ${succeededTests} tests succeeded, ${
        testsToRun - succeededTests
      } tests failed`
    )
    if (testsToRun !== succeededTests) {
      return false
    }
    return true
  }

  private getSpecificTests() {
    let specificTests = new Array<string>()
    for (let i = 2; i < process.argv.length; ++i) {
      specificTests.push(process.argv[i])
    }
    if (specificTests.length === 0) {
      specificTests = Array.from(this.tests.keys())
    }
    return specificTests
  }
}
