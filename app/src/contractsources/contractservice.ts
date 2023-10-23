import { ContractsRepoReader } from './contractsreporeader'

export class ContractService {
  private readonly contractsRepoReader = new ContractsRepoReader()
  private readonly contractCodes = new Map<string, string>()

  public async readContracts() {
    const content = await this.contractsRepoReader.readContractsDir('contracts')
    for (const contract of content) {
      await this.readContract(contract.name, contract.path)
    }
  }

  private async readContract(name: string, path: string) {
    const content = await this.contractsRepoReader.readContractFile(
      `${path}/src/lib.rs`
    )
    if (content !== undefined) {
      this.contractCodes.set(name, content)
    }
  }

  public getContractCode(contractName: string): string | undefined {
    return this.contractCodes.get(contractName)
  }
}
