import { LotteryCodeGen } from './lotterycodegen'
import { VotingCodeGen } from './votingcodegen'

export class CodeGen {
  private header = ''
  private contractCode = ''

  public generateHeader(author: string, license: string): string {
    this.header = ''
    if (author !== '') {
      this.header = `//author: ${author}\n`
    }
    if (license !== '') {
      this.header += `//license: ${license}\n`
    }
    return this.header
  }

  public generateContractCode(trait: string, name: string): string {
    this.contractCode = ''
    if (trait === 'Lottery') {
      const lotteryCodeGen = new LotteryCodeGen()
      this.contractCode = lotteryCodeGen.generateCode(name)
    } else if (trait === 'Voting') {
      const votingCodeGen = new VotingCodeGen()
      this.contractCode = votingCodeGen.generateCode(name)
    }
    return this.contractCode
  }

  public getCode(): string {
    return this.header + '\n' + this.contractCode
  }
}
