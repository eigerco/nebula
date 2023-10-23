import fetch from 'node-fetch'

export class ContractsRepoReader {
  private readonly ghUrl =
    'https://api.github.com/repos/eigerco/nebula/contents'

  public async readContractsDir(path: string) {
    const url = `${this.ghUrl}/${path}`
    console.debug(`fetching ${url}`)
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        Accept: 'application/json',
      },
    })
    const result = await response.json()
    return result
  }

  public async readContractFile(path: string) {
    const url = `${this.ghUrl}/${path}`
    console.debug(`fetching ${url}`)
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        Accept: 'application/json',
      },
    })
    if (response.ok) {
      const result = await response.json()
      const content = atob(result.content)
      return content
    }
    return undefined
  }
}
