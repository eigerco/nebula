import { RaffleCodeGen } from './rafflecodegen'
import { VotingCodeGen } from './votingcodegen'

export class InvokeCommandGen {
  generateInvokeCommand(trait: string, name: string, params: any[]): string {
    if (trait === 'Raffle') {
      const raffleCodeGen = new RaffleCodeGen()
      return raffleCodeGen.generateInvokeCommand(name, params)
    }
    if (trait === 'Voting') {
      const votingCodeGen = new VotingCodeGen()
      return votingCodeGen.generateInvokeCommand(name, params)
    }
    return ''
  }
}
