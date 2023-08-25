export class ContractsCodeGen {
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

  public generateContractCode(
    originalCode: string,
    contractName: string
  ): string {
    if (originalCode === undefined) {
      return ''
    }
    this.contractCode = originalCode

    const contractIdx = this.contractCode.indexOf('#[contract]')
    if (contractIdx !== -1) {
      const contractNameIdx = this.contractCode.indexOf(
        'pub struct',
        contractIdx
      )
      if (contractNameIdx !== -1) {
        const endLineIdx = this.contractCode.indexOf('\n', contractNameIdx)
        this.contractCode = this.contractCode.replace(
          this.contractCode.substring(contractNameIdx + 11, endLineIdx),
          contractName
        )
      }
    }

    const contractimplIdx = this.contractCode.indexOf('#[contractimpl]')
    if (contractimplIdx !== -1) {
      const contractNameIdx = this.contractCode.indexOf(
        'impl',
        contractimplIdx + 15
      )
      if (contractNameIdx !== -1) {
        const endLineIdx = this.contractCode.indexOf('{', contractNameIdx)
        this.contractCode = this.contractCode.replace(
          this.contractCode.substring(contractNameIdx + 5, endLineIdx),
          contractName + ' '
        )
      }
    }
    return this.contractCode
  }

  public getCode(): string {
    return this.header + '\n' + this.contractCode
  }

  public getInvokes(commandId: any) {
    const contractimplIdx = this.contractCode.indexOf('#[contractimpl]')
    const traitEndIdx = this.contractCode.indexOf('impl ', contractimplIdx + 25)
    const lenses = []
    if (contractimplIdx !== -1) {
      let funIdx = this.contractCode.indexOf('pub fn', contractimplIdx)
      while (funIdx !== -1) {
        const lineNo =
          this.contractCode.substring(0, funIdx).split(/\r\n|\r|\n/).length + 3 // +3 because of header
        const range = {
          startLineNumber: lineNo,
          startColumn: 1,
          endLineNumber: lineNo + 1,
          endColumn: 1,
        }
        const funName = this.getFunName(funIdx)
        const funParams = this.getFunParams(funIdx)
        const command = {
          id: commandId(funName, funParams),
          title: 'invoke',
        }
        lenses.push({
          range,
          id: 'invoke',
          command,
        })
        funIdx = this.contractCode.indexOf('pub fn', funIdx + 1)
        if (traitEndIdx !== -1 && funIdx >= traitEndIdx) {
          break
        }
      }
    }
    return {
      lenses,
      dispose: () => {},
    }
  }

  private getFunName(funIdx: number) {
    const funNameEndIdx = this.contractCode.indexOf('(', funIdx)
    return this.contractCode.substring(funIdx + 7, funNameEndIdx)
  }

  private getFunParams(funIdx: number) {
    const funEndIdx = this.contractCode.indexOf(')', funIdx)
    let params = ''
    if (funEndIdx !== -1) {
      // omit the first param (env)
      let paramBegIdx = this.contractCode.indexOf(',', funIdx)

      paramBegIdx = this.contractCode.indexOf(',', paramBegIdx) + 1
      let paramEndIdx = this.contractCode.indexOf(':', paramBegIdx)
      let paramTypeEndIdx = this.contractCode.indexOf(',', paramEndIdx)

      while (paramEndIdx !== -1) {
        if (paramTypeEndIdx >= funEndIdx) {
          paramTypeEndIdx = funEndIdx
        }
        const paramName = this.contractCode
          .substring(paramBegIdx, paramEndIdx)
          .trimStart()
          .trimEnd()
        const paramType = this.contractCode
          .substring(paramEndIdx + 1, paramTypeEndIdx)
          .trimStart()
          .trimEnd()
        params += `--${paramName} ${this.getDefaultValueForType(
          paramName,
          paramType
        )} \\\n\t`

        paramBegIdx = this.contractCode.indexOf(',', paramEndIdx) + 1
        paramEndIdx = this.contractCode.indexOf(':', paramBegIdx)
        paramTypeEndIdx = this.contractCode.indexOf(',', paramEndIdx)
        if (paramEndIdx >= funEndIdx) {
          break
        }
      }
      params = params.substring(0, params.length - 3)
    }
    return params
  }

  private getDefaultValueForType(name: string, type: string) {
    if (type === 'Address') {
      return `{${name}_address}`
    }
    if (type.startsWith('u') || type.startsWith('i')) {
      return '1'
    }
    return ''
  }
}
