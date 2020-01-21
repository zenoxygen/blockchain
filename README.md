# blockchain [![Build Status](https://travis-ci.com/zenoxygen/blockchain.svg?branch=master)](https://travis-ci.com/zenoxygen/blockchain)

A basic blockchain for educational purposes.

## Overview

1. Create a genesis block with one transaction where:
  - Alice receives 50 coins and Bob receives 25 coins
2. Mine that block
3. Add it to the blockchain
4. Create a new block with two transactions where:
  - Charlie receives 5 coins
  - Alice sends 30 coins to herself and sends 20 coins to Bob
5. Mine that block
6. Add it to the blockchain

## Dependencies

- [hex](https://crates.io/crates/hex), for encoding/decoding data into/from hexadecimal representation
- [crypto-hash](https://crates.io/crates/crypto-hash), for cryptographic hash functions

## Run

`cargo run`

## More details

### Block hashing

1. Concatenate together all the bytes composing the block's fields (aside from the hash field)
2. Generate unique data fingerprint: the hash

### Difficulty

Difficulty is expressed in this way:

- The first *n* bytes of the hash that must be zero
- The number of bits or bytes as the beginning of the hash that must be zero

### Mining

1. Generate new nonce (an arbitrary number that can be used just once)
2. Hash bytes (computationally heavy step)
3. Check hash against difficulty
  - Insufficient? Go back to step 1
  - Sufficient? Continue to step 4
4. Add block to the blockchain
