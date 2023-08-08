import { ContractsRepoReader } from './contractsreporeader'

export class ContractService {
  private readonly contractsRepoReader = new ContractsRepoReader()
  private readonly contractsContent = new Map<string, string>()

  public async readContracts() {
    const content = await this.contractsRepoReader.readContractsDir('contracts')
    for (const contract of content) {
      await this.getContract(contract.name, contract.path)
    }
  }

  private async getContract(name: string, path: string) {
    const content = await this.contractsRepoReader.readContractFile(
      `${path}/src/lib.rs`
    )
    this.contractsContent.set(name, content)
  }

  public getContractsContent(contractName: string): string | undefined {
    return this.contractsContent.get(contractName)
  }
}
