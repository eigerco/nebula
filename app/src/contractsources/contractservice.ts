import { ContractFileReader } from './contractfilereader'

export class ContractService {
  private readonly contractsFileReader = new ContractFileReader()
  private readonly contractsContent = new Map<string, string>()

  public async readContracts() {
    let content = await this.contractsFileReader.readContractFile('contracts/voting/src/lib.rs')
    this.contractsContent.set('voting', content)
    content = await this.contractsFileReader.readContractFile('contracts/raffle/src/lib.rs')
    this.contractsContent.set('raffle', content)
  }

  public getContractsContent(contractName: string): string | undefined {
    return this.contractsContent.get(contractName)
  }
}
