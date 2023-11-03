# Governance contract

This contract provides the implementation of a stake controlled DAO that allows participants vote on:

* Code upgrades.
* Curator changes.
* General proposals with arbitrary comments.

## Features

This contract implements the following layers:

* Voting layer. Delegated to the "voting" contract of this catalog, which is called via cross contract calls.
* Staking layer. Implemented on this contract. It allows participants to stake and withdraw funds any time.
* Governance layer. This contract itself. It manages the membership, like join or leave operations, which implies also funds management via the staking layer.

The governance contract maintains an internal balance of all the participant staked funds. The staked balance represents the voting power of each participant at a given moment.

Only when a proposal is finally executed by the proposer, the final results of participation are stored in the voting contract storage, as the voting power (staking) can change per each participant during voting as they stake,withdraw or leave the DAO.

All participants needs to be "whitelisted" by the curator before they can create or vote proposals.

The current voting mechanism requires a minimum participation configured at DAO initial setup in order to consider a proposal "approved". Voting a proposal can only mean a positive vote.

## Importing with nebula-importer

````toml
[package.metadata.nebula.imports]
governance = "ghcr.io/eigerco/nebula/contracts/governance:latest"
````

## Contract methods

* `init` - contract initialization,
* `register` - each user must register before he can join,
* `join` - participants can join the DAO by invoking this function,
* `stake` - participants can increase their staked amounts any time,
* `leave` - participants can leave anytime, withdrawing all amounts,
* `withdraw` - participants can withdraw their staked amounts any time,
* `whitelist` - only curator can invoke this function for whitelisting a participant,
* `new_proposal` - allows any whitelisted participant to create a new proposal,
* `vote` - any whitelisted participant can vote on a proposal,
* `execute_proposal` - only a whitelisted participant, who is the proposer, can execute the given proposal.


## Using the contract

### Contract initialization

To initialize the contract the `init` method needs to be called with following arguments:
* `curator` - the account address that can whitelist participants,
* `token` - the token that accomplishes the token interface and this DAO uses as base currency,
* `voting_period_secs` - the time a created proposal is open for voting,
* `target_approval_rate_bps` - the default max number of participation for new proposals,
* `salt` - a needed salt for generating addresses for the deployed contracts.
```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_private_key} \
  --network ${network} \
  -- \
  init \
  --curator ${curator_address} \
  --token ${token_contract_id} \
  --voting_period_secs 3600 \
  --target_approval_rate_bps 5000 \
  --salt ef
```
Contract can only be initialized once.

### User registration

Each user must register first before he can buy the join the DAO. The `register` method takes one argument:
* `by` - address of a player

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${player_private_key} \
  --network ${network} \
  -- \
  register \
    --by ${player_address}
```

### Joining a DAO

To join the DAO user need to call the `join` method with following arguments:
* `participant_addr` - the participant address,
* `amount` - the initial amount this user wants to participate with.

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${participant_private_key} \
  --network ${network} \
  -- \
  join \
    --participant_addr ${participant_address} \
    --amount 1000
```

### Staking

Participants can increase their stake by calling `stake` function with the following arugments:
* `participant_addr` - the participant address,
* `amount` - the initial amount this user wants to participate with.

```bash
 soroban contract invoke \
  --id ${contract_id} \
  --source ${participant_private_key} \
  --network ${network} \
  -- \
  stake \
    --participant ${participant_address} \
    --amount 100
```

### Leaving DAO

Participants can leave anytime, withdrawing all amounts by calling `leave` functions with arguments below. Once the leave, they need to be whitelisted again. 
* - `participant` - the participant address.

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${participant_private_key} \
  --network ${network} \
  -- \
  leave \
    --participant ${participant_address}
```

### Withdrawing funds

Participants can withdraw their staked amounts any time by calling `withdraw` with arguments:
* `participant` - the participant address,
* `amount` - the initial amount this user wants to participate with.

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${participant_private_key} \
  --network ${network} \
  -- \
  withdraw \
    --participant ${participant_address} \
    --amount 100
```

### Whitelisting accounts

Only curator can invoke the `whitelist` function for whitelisting a participant.
* `participant_addr` - the participant address for whitelisting.

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_private_key} \
  --network ${network} \
  -- \
  whitelist \
    --participant ${participant_address}
```

### Creating new proposal

Any whitelisted participant can create new proposal by calling `new_proposal` with following arguments:
* `participant` - the proposer who is creating this proposal,
* `id` -  the unique ID of the proposal. This can be taken from external systems,
* `payload` - the [`voting_contract::ProposalPayload`] , that represents a Proposal kind + its respective payload
    
```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${participant_private_key} \
  --network ${network} \
  -- \
  new_proposal \
    --participant ${participant_address} \
    --id 1 \
    --payload '{"Comment": "efe"}'
```

### Voting on a proposal

Any whitelisted participant can vote on a proposal by calling `vote` with 2 arguments:
* `participant` - The proposer who is creating this proposal.
* `id` -  The unique ID of the proposal.

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${participant_private_key} \
  --network ${network} \
  -- \
  vote \
    --participant ${participant_address} \
    --id 1
```

### Excuting proposals

Only a whitelisted participant, who is the proposer, can execute the given proposal with `execute_proposal` function with following arguments:
* `participant` - the proposer who is executing this proposa,
* `id` -  the unique ID of the proposal.

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${participant_private_key} \
  --network ${network} \
  -- \
  execute_proposal \
  --participant ${participant_address} \
  --id 1
```
