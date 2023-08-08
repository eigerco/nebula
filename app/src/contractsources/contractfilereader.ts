import fetch from 'node-fetch'

export class ContractFileReader {
  public async readContractFile(path: string) {
    const str = `https://api.github.com/repos/eigerco/nebula/contents/${path}`
    console.debug(`fetching ${str}`)
    const response = await fetch(`https://api.github.com/repos/eigerco/nebula/contents/${path}`, {
      method: 'GET',
      headers: {
        Accept: 'application/json',
      },
    })
    const result = (await response.json())
    const content = atob(result.content)
    return content
  }
}
