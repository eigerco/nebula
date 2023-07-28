import React from 'react'
import { DefaultParams } from './DefaultParams'
import { RaffleParams } from './RaffleParams'
import { VotingParams } from './VotingParams'

export function ContractParams({ contractTrait, updateParams }: any) {
  if (contractTrait === 'Raffle') {
    return <RaffleParams updateParams={updateParams} />
  }
  if (contractTrait === 'Voting') {
    return <VotingParams updateParams={updateParams} />
  }
  return <DefaultParams updateParams={updateParams} />
}
