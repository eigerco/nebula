import { LotteryCodeGen } from "./lotterycodegen";
import { VotingCodeGen } from "./votingcodegen";

export class InvokeCommandGen {
  generateInvokeCommand(type: string, name: string, params: any[]): string {
    if (type === 'Lottery') {
      const lotteryCodeGen = new LotteryCodeGen()
      return lotteryCodeGen.generateInvokeCommand(name, params)
    }
    if (type === 'Voting') {
      const votingCodeGen = new VotingCodeGen()
      return votingCodeGen.generateInvokeCommand(name, params)
    }
    return ''
  }
}