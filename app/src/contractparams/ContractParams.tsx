import React from 'react'
import { DefaultParams } from './DefaultParams'
import { LotteryParams } from './LotteryParams'
import { VotingParams } from './VotingParams'

export function ContractParams({ contractTrait, updateParams }: any) {
  if (contractTrait === 'Lottery') {
    return <LotteryParams updateParams={updateParams} />
  }
  if (contractTrait === 'Voting') {
    return <VotingParams updateParams={updateParams} />
  }
  return <DefaultParams updateParams={updateParams} />
}
