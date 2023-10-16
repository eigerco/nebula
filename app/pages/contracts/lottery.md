# Lottery contract

The Lottery contract implements a lottery. Lottery creator specifies how many numbers players will need to select, from what range (always starting from 1), thresholds with available prizes for a given number of properly selected numbers, and minimum number of players required for the lottery to be played. After the lottery is created each user can buy unlimited number of tickets where he selects his own numbers. At the end the specified number of numbers are randomly drawn, and players receive prizes based on their selection and predefined prize thresholds. Unspend tokens from the lottery pool are carried over to the next lottery.

## Features

* lottery creator specifies the lottery rules - how many numbers players will need to select, from what range and what are the prizes
* everyone can join the lottery by buying the ticket and selecting numbers,
* each player can buy many tickets - the more he buys, the more chances of win he has,
* unspend tokens from the lottery pool are carried over to the next lottery,
* sometimes lottery can have no winners,
* once the lottery is finished, no more tickets can be bought until new lottery is created


## Importing with nebula-importer

````toml
[package.metadata.nebula.imports]
lottery = "ghcr.io/eigerco/nebula/contracts/lottery:latest"
````

## Contract methods

* `init` - contract initialization,
* `create_lottery` - creates new lottery, can be called each time previous lottery is finished,
* `buy_ticket` - users can call this method to buy tickets for the lottery,
* `play_lottery` - launches the lottery,
* `check_lottery_results` - returns results for a given lottery,
* `get_pool_balance` - returns current lottery pool balance, can be only called by an admin.

## Using the contract

### Contract initialization

To initialize the contract the `init` method needs to be called with 4 arguments:
* `admin` - admin account address.
* `token` - the asset contract address we are using for this lottery. See [token interface](https://soroban.stellar.org/docs/reference/interfaces/token-interface).
* `ticket_price` - unitary ticket price for the current lottery.
* `number_of_numbers` - number of numbers possible to select by players
* `max_range` - right boundary of the range players will select numbers from (1, max_range)
* `thresholds` - thresholds with prizes for correctly selected numbers (specified as percentage of the pool balance)
* `min_players_count` - minimum number of players needed to play the lottery

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_private_key} \
  --network ${network} \
  -- \
  init \
    --admin ${admin_address} \
    --token ${token_contract_id} \
    --ticket_price 5001 \
    --number_of_numbers 5 \
    --max_range 50 \
    --thresholds '{"5": 30, "4": 15, "3": 10}' \
    --min_players_count 10
```
Contract can only be initialized once.

#### Setting thresholds

Each threshold is a pair of two numbers:
* a number of correctly selected numbers for which the prize will be paid,
* a percentage of pool balance to be paid for players that will correctly select the above defined number of numbers.

In the above example there are 3 thresholds defined:
* `"5": 30` - meaning: 30% of the pool balance will be paid to every player who properly selected 5 numbers,
* `"4": 15` - 15% of the pool balance pay out for correctly selected 4 numbers,
* `"3": 10` - 10% of the pool balance pay out for correctly selected 3 numbers.

Only players that have correctly selected 3, 4 or 5 numbers will have the prizes paid out.

There could be cases where the total prize amount exceeds the current balance in the pool. For instance, using the previous example, if four players correctly select five numbers each, they would each be entitled to 30% of the pool, totaling 120%. This situation is not feasible, so in such cases, the prize thresholds will be adjusted to ensure that the total prize payout remains equal to or less than the current balance of the lottery pool. However, the proportions of the prizes will still be maintained.

### Creating a new lottery

First lottery is always created during contract initalization. However, when it ends it is possible to create a new one with new specification using the method `create_lottery` which has the following arguments:
* `ticket_price` - unitary ticket price for the current lottery.
* `number_of_numbers` - number of numbers possible to select by players
* `max_range` - right boundary of the range players will select numbers from (1, max_range)
* `thresholds` - thresholds with prizes for correctly selected numbers (specified as percentage of the pool balance)
* `min_players_count` - minimum number of players needed to play the lottery

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_private_key} \
  --network ${network} \
  -- \
  create_lottery \
    --ticket_price 5001 \
    --number_of_numbers 5 \
    --max_range 50 \
    --thresholds '{"5": 30, "4": 15, "3": 10}'\
    --min_players_count 10
```
Function also publishes an event with lottery specification: lottery number, number of numbers to select, max range, thresholds and ticket prize.

### Buying tickets

Each user that wants to take a part in the lottery need to buy a ticket first. To do this a `buy_ticket` method need to be invoked with the following arguments:
* `by` - the address of the user that is buying the ticket,
* `ticket` - the selected numbers by the player - they all have to be within ranges specified during lottery creation and their count should also match.


```bash
soroban contract invoke \
    --id ${contract_id} \
    --source ${player_private_key} \
    --network ${network} \
    -- \
    buy_ticket \
      --by ${player_address} \
      --ticket '[5, 10, 13, 22, 47]'
```
User can buy as much tickets as he wants.

### Playing the lottery

Only user with `admin` role can start the lottery with the `play_lottery` method that requires one argument:
* `random_seed` - a seed used to initialize random number generator

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_address} \
  --network ${network} \
  -- \
  play_lottery \
    --random_seed 1234
```
After this method is invoked, random numbers are drawn, and players with the highest number of correct matches receive prizes in accordance with the specified thresholds. Any remaining funds in the lottery pool are rolled over to the next lottery. It's important to note that no further tickets can be purchased for the current lottery at this point. However, an admin has the ability to create a new lottery for future participation.

### Checking lottery results

Each lottery has its unique number which can be later used for checking its results. The method `check_lottery_results` returns lottery results for a given lottery number which is its argument:
* `lottery_number`

```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_address} \
  --network ${network} \
  -- \
  check_lottery_results \
    --lottery_number 1
```
In case wrong lottery number is set, error will be returned.

### Checking lottery pool balance

Admin can check the current lottery pool balance by invoking `get_pool_balance` method.
```bash
soroban contract invoke \
  --id ${contract_id} \
  --source ${admin_address} \
  --network ${network} \
  -- \
  get_pool_balance
```
This method requires no arguments.
