# ICP Wallet Smart Contract

`Overview`

This project implements a simple wallet smart contract for the Internet Computer Protocol (ICP). The smart contract allows users to manage a wallet with basic functionalities such as sending and receiving tokens, checking balances, and setting the wallet owner.

# Features

Initialize a wallet with an owner ->

`dfx canister call icp_wallet_backend set_owner '("your-principal-id")'`

you can find your principal id via : `dfx identity get-principal`
response : tuvpa-y7egx-ef76w-ajpdd-qkyof-ohhyl-kk4os-62z75-bywy7-rhzfj-uqe

Set and get wallet balance

set wallet balance : `dfx canister call icp_wallet_backend set_balance '(1000 : nat64)'`
response : (variant { 17_724 })

get wallet balance : `dfx canister call icp_wallet_backend get_balance`
response : (1_000 : nat64)

Send tokens to other addresses : `dfx canister call icp_wallet_backend send_tokens '(100 : nat64, "recipient_address")'`
response : Remaining balance after sending: 900

`now our remaining balance would be 900`

Receive tokens : `dfx canister call icp_wallet_backend receive_tokens '(99 : nat64)'`
response : The balance after recieving: 999

`now the balance would be 999`

Change wallet ownership
`dfx canister call icp_wallet_backend set_owner '("your-principal-id")'`

# SETUP

`git clone https://github.com/ashutoshnautiyal7/icp_wallet`

`cd icp_wallet`

`dfx start --background`
`dfx build icp_wallet_backend`
`dfx deploy icp_wallet_backend`

# Security Considerations

The smart contract includes basic authorization checks to ensure only the owner can perform sensitive actions.
In a production environment, additional security measures should be implemented, such as secure key management and potentially multi-signature functionality.

# Testing

To run the unit tests for the smart contract:
`cargo test`

# response :

Compiling icp_wallet_backend v0.1.0 (/home/ashutosh/Documents/freelance/Assignments/icp_wallet_project/icp_wallet/src/icp_wallet_backend)
Finished `test` profile [unoptimized + debuginfo] target(s) in 1.39s
Running unittests src/lib.rs (target/debug/deps/icp_wallet_backend-2811513c2cfda115)

running 8 tests
test tests::test_get_owner ... ok
test tests::test_get_balance ... ok
test tests::test_send_tokens_insufficient_balance ... ok
test tests::test_receive_tokens ... ok
test tests::test_send_tokens_sufficient_balance ... ok
test tests::test_set_and_get_balance ... ok
test tests::test_set_owner ... ok
test tests::test_unauthorized_access ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
