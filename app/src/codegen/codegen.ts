import { RaffleCodeGen } from './rafflecodegen'
import { VotingCodeGen } from './votingcodegen'

export class CodeGen {
  private header = ''
  private contractCode = ''

  public generateHeader(author: string, license: string): string {
    this.header = ''
    if (author !== '') {
      this.header = `// author: ${author}\n`
    }
    if (license !== '') {
      this.header += `// license: ${license}\n`
    }
    return this.header
  }

  public generateContractCode(trait: string, name: string): string {
    this.contractCode = ''
    if (trait === 'Raffle') {
      const raffleCodeGen = new RaffleCodeGen()
      this.contractCode = raffleCodeGen.generateCode(name)
    } else if (trait === 'Voting') {
      const votingCodeGen = new VotingCodeGen()
      this.contractCode = votingCodeGen.generateCode(name)
    }
    return this.contractCode
  }

  public getCode(): string {
    return this.header + '\n' + this.contractCode
  }

  public getInvokes(trait: string, commandId: any) {
    if (trait === 'Raffle') {
      const raffleCodeGen = new RaffleCodeGen()
      return raffleCodeGen.getInvokes(commandId)
    } else if (trait === 'Voting') {
      const votingCodeGen = new VotingCodeGen()
      return votingCodeGen.getInvokes(commandId)
    }
    return {}
  }
}
