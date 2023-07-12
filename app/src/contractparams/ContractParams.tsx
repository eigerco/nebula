import React from 'react'
import { DefaultParams } from './DefaultParams'
import { LotteryParams } from './LotteryParams'
import { VotingParams as VotingParams } from './VotingParams'

export function ContractParams({ contractType, updateParams }: any) {
  if (contractType === 'Lottery') {
    return <LotteryParams updateParams={updateParams} />
  }
  if (contractType === 'Voting') {
    return <VotingParams updateParams={updateParams} />
  }
  return <DefaultParams updateParams={updateParams} />
}
