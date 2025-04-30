# Emulating Solidity interfaces in Stellar

There is no equivalent of a Solidity interface in Soroban (as well as in Rust).
But some functionality can be emulated with Rust trait system.

## Setup

Install [`stellar` cli](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup).

```sh
cd contracts/contract_a
stellar contract build
```

```sh
cd contracts/contract_b
stellar contract build
cargo test
```

## License

Apache 2.0
