import { LotteryCodeGen } from './lotterycodegen'
import { VotingCodeGen } from './votingcodegen'

export class InvokeCommandGen {
  generateInvokeCommand(trait: string, name: string, params: any[]): string {
    if (trait === 'Lottery') {
      const lotteryCodeGen = new LotteryCodeGen()
      return lotteryCodeGen.generateInvokeCommand(name, params)
    }
    if (trait === 'Voting') {
      const votingCodeGen = new VotingCodeGen()
      return votingCodeGen.generateInvokeCommand(name, params)
    }
    return ''
  }
}
