# Soroban Savings Goal Contract

A Soroban smart contract for creating and tracking on-chain savings goals
on the Stellar network. Each address can own multiple independent, named
savings goals, protected by owner authentication, with support for
optional deadlines and an early-exit path with a penalty.

## Features

- Multiple named savings goals per address (e.g. `"vacation"`, `"emergency"`)
- Owner-only actions enforced via Soroban's `require_auth()`
- Optional deadlines on goals
- Emergency withdrawal before target is reached, with a 10% penalty
- Events emitted for goal creation, deposits, withdrawals, and emergency withdrawals
- Typed contract errors instead of panics, for clean client-side error handling

## Functions

### `create_goal(owner: Address, goal_name: Symbol, target: i128, deadline: Option<u64>) -> Result<(), Error>`

Creates a new savings goal for `owner`, identified by `goal_name`. Requires
`owner`'s authorization. Fails with `InvalidTarget` if `target <= 0`, with
`InvalidDeadline` if `deadline` is in the past, and with `GoalAlreadyExists`
if a goal with that name already exists for this owner.

### `deposit(owner: Address, goal_name: Symbol, amount: i128) -> Result<(), Error>`

Adds `amount` to the goal's balance. Requires `owner`'s authorization. Fails
with `InvalidAmount` if `amount <= 0`, or `GoalNotFound` if the goal doesn't exist.

### `get_balance(owner: Address, goal_name: Symbol) -> Result<i128, Error>`

Returns the current balance of the specified goal.

### `get_target(owner: Address, goal_name: Symbol) -> Result<i128, Error>`

Returns the target amount of the specified goal.

### `get_deadline(owner: Address, goal_name: Symbol) -> Result<Option<u64>, Error>`

Returns the goal's deadline as a ledger timestamp, if one was set.

### `get_remaining_to_target(owner: Address, goal_name: Symbol) -> Result<i128, Error>`

Returns how much more is needed to reach the target.

### `withdraw(owner: Address, goal_name: Symbol) -> Result<i128, Error>`

Withdraws the full balance once it meets or exceeds the target, and removes
the goal. Requires `owner`'s authorization. Fails with `TargetNotReached` if
the balance hasn't hit the target yet.

### `emergency_withdraw(owner: Address, goal_name: Symbol) -> Result<i128, Error>`

Withdraws the current balance before the target is reached, minus a 10%
penalty, and removes the goal. Requires `owner`'s authorization. Fails with
`NothingToWithdraw` if the balance is zero.

## Errors

| Error               | Meaning                                             |
| ------------------- | --------------------------------------------------- |
| `InvalidTarget`     | Target amount must be greater than zero             |
| `InvalidAmount`     | Deposit amount must be greater than zero            |
| `TargetNotReached`  | Cannot withdraw until balance >= target             |
| `GoalAlreadyExists` | A goal with this name already exists for this owner |
| `GoalNotFound`      | No goal with this name exists for this owner        |
| `Unauthorized`      | Caller is not the goal owner                        |
| `InvalidDeadline`   | Deadline must be in the future                      |
| `NothingToWithdraw` | Balance is zero, nothing to emergency-withdraw      |

## Tech Stack

- Rust
- Soroban SDK 22.x
- Stellar

## Building and testing locally

```bash
cargo test
cargo build --target wasm32-unknown-unknown --release
```

## Deploying to testnet

```bash
stellar keys generate deployer --network testnet
stellar keys fund deployer --network testnet
stellar contract build
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/savings_goal.wasm \
  --source deployer \
  --network testnet
```
