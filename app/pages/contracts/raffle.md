# Raffle contract

The Raffle contract implements a simple raffle: users buy tickets and one or more of them are randomly selected as winners. Prizes are paid out from the pool filled with tokens from bought tickets, evenly distributed among all winners.

## Features
* everyone can join the raffle by buying the ticket,
* each player can buy many tickets - the more he buys, the more chanes of win he has,
* there is always a winner in the raffle,
* once the raffle is finished, no more tickets can be bought.

## Importing with nebula-importer
````toml
[package.metadata.nebula.imports]
raffle = "ghcr.io/eigerco/nebula/contracts/raffle:latest"
````

## Contract methods
* `init` - contract initialization,
* `buy_ticket` - users can call this method to buy tickets for the raffle,
* `play_raffle` - launches the raffle.

## Using the contract
### Contract initialization
To initialize the contract the `init` method needs to be called with 4 arguments:
* `admin` - address of the admin account
* `token` - ID of the token contract
* `max_winners_count` - number of players that could win a raffle (1 is minimum)
* `ticket_price` - price of the ticket

```
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_private_key} \
  --network ${network} \
  -- \
  init \
    --admin ${admin_address} \
    --token ${token_contract_id} \
    --max_winners_count 1 \
    --ticket_price 5001
```
Contract can only be initialized once.
### Buying tickets
Each user that wants to take a part in the raffle need to buy a ticket first. To do this a `buy_ticket` method need to be invoked with the following argument:
* `by` - address of a player

```
soroban contract invoke \
    --id ${contract_id} \
    --source ${player_private_key} \
    --network ${network} \
    -- \
    buy_ticket \
      --by ${player_address}
```
Ticket can only be bought once by a user.
### Playing the raffle
Only user with `admin` role can start the raffle with the `play_raffle` method that requires one argument:
* `random_seed` - a seed used to initialize random number generator

```
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_address} \
  --network ${network} \
  -- \
  play_raffle \
    --random_seed 1234
```
After this method is called a winner(s) is randomly selected and the raffle prize is paid out. Additionaly an event with winner(s) address(es) and the value of pay out is emitted. No more tickets can be bought after the raffle has been played.
