# Marketplace Contract

The marketplace contract enables the creation and management of listings for various assets. Users can buy, update, pause, and remove listings.

## Features

- Create and manage listings for assets.
- Buy assets listed in the marketplace.
- Update listing prices.
- Pause listings. So no buy operation can be performed.
- Unpause listings. So resuming buying operations.
- Remove listings from the marketplace. Returning the balance to the original owners (sellers).

## Using the contract

### Contract Initialization

To initialize the contract, use the following command:

```shell
soroban contract invoke \
  --id ${contract_id} \
  --source admin \
  --network ${network} \
  -- \
  init \
  --token ${token_address}\
  --admin ${admin_address}
```
### Creating a Listing

To create a new listing for an asset in the marketplace, use the following command:

```shell
soroban contract invoke \
  --id ${contract_id} \
  --source trader_1 \
  --network ${network} \
  -- \
  create_listing \
  --seller ${trader_address}  \
  --asset ${asset_address} \
  --price 100 \
  --quantity 1
```

### Getting a Listing

Retrieve information about a specific listing in the marketplace with this command:

```shell
soroban contract invoke \
  --id ${contract_id} \
  --source trader_2 \
  --network ${network} \
  -- \
  get_listing \
  --id ${id}
```

### Buying a Listing

To purchase a listed asset from the marketplace, use the following command:

```shell
soroban contract invoke \
  --id ${contract_id} \
  --source trader_2 \
  --network ${network} \
  -- \
  buy_listing \
  --buyer ${trader_address} \
  --id ${id}
```

### Updating the price of a listing

Update the price of a listing with this command:

```shell

soroban contract invoke \
  --id ${contract_id} \
  --source trader_2 \
  --network ${network} \
  -- \
  update_price \
  --id ${id} \
  --new_price 150
```

### Pausing a listing

Temporarily deactivate a listing with the following command:

```shell
soroban contract invoke \
  --id ${contract_id} \
  --source trader_2 \
  --network ${network} \
  -- \
  pause_listing \
  --id ${id}
```

### Unpausing a listing

Temporarily deactivate a listing with the following command:

```shell
soroban contract invoke \
  --id ${contract_id} \
  --source trader_2 \
  --network ${network} \
  -- \
  unpause_listing \
  --id ${id}
```

### Removing a listing

Remove a listing from the marketplace with this command:

```shell
soroban contract invoke \
  --id ${contract_id} \
  --source trader_2 \
  --network ${network} \
  -- \
  remove_listing \
  --id ${id} \
```
This updated documentation provides an overview of the features of the marketplace contract and instructions on how to use its various methods for listing, buying, updating, pausing, and removing assets from the marketplace.

