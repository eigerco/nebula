## Voting Contract

The voting contract implements a proposal voting strategy: allows admins to create and users to vote on multiple proposals.

## Features

* Proposals are deadline bound.
* Proposals are identified by an unique ID, that might be maintained by external applications.
* Anyone can vote on a proposal.


## Importing with nebula-importer

````toml
[package.metadata.nebula.imports]
voting = "ghcr.io/eigerco/nebula/contracts/voting:latest"
````

## Contract methods

* `init` - contract initialization.
* `create_proposal` - create proposal with defaults.
* `create_custom_proposal` - create a custom proposal.
* `vote` - vote on a proposal.

## Using the contract

### Contract initialization

To initialize the contract the `init` method needs to be called with 4 arguments:
- `admin` - The address that can create proposals.
- `voting_period_secs` - The default number of seconds of proposals lifetime for new proposals.
- `target_approval_rate_bps` - The default required approval rate in basic points for new proposals.
- `total_voters` - The default max number of voters for new proposals.

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_private_key} \
  --network ${network} \
  -- \
  init \
    --admin ${admin_address} \
    --voting_period_secs 60 \
    --target_approval_rate_bps 1 \
    --total_voters 100
```
Contract can only be initialized once.

### Creating Proposals

And admin can create a proposal with the defaults.
* `id` - id of the proposal

```bash
soroban contract invoke \
    --id ${contract_id} \
    --source ${admin_private_key} \
    --network ${network} \
    -- \
    create_proposal \
      --id ${proposal_id}
```
A proposal id can only be used once.

### Voting for a proposal

Once a proposal is created, voters can vote.
* `voter` - The voter address, which should match with transaction signatures.
* `id` - The unique identifier of the proposal.

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${voter_address} \
  --network ${network} \
  -- \
  vote \
    --voter ${voter_address} \
    --id 1234
```