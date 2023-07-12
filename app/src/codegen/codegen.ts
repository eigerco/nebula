import { LotteryCodeGen } from "./lotterycodegen";
import { VotingCodeGen } from "./votingcodegen";

export class CodeGen {
  private header = "";
  private contractCode = "";

  public setHeader(author: string, license: string) {
    this.header = "";
    if (author !== "") {
      this.header = `//author: ${author}\n`;
    }
    if (license !== "") {
      this.header += `//license: ${license}\n`;
    }
  }

  public setContractCode(type: string, name: string, params: any[]) {
    this.contractCode = "";
    if (type === "Lottery") {
      const lotteryCodeGen = new LotteryCodeGen();
      this.contractCode = lotteryCodeGen.generateCode(name);
    } else if (type === "Voting") {
      const votingCodeGen = new VotingCodeGen();
      this.contractCode = votingCodeGen.generateCode(name);
    }
  }

  public generateCode(): string {
    return this.header + "\n" + this.contractCode;
  }
}
