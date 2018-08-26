# Bitcoin RPC Client

[![Build Status](https://travis-ci.com/coblox/bitcoincore-rpc_rust_client.svg?branch=master)](https://travis-ci.com/coblox/bitcoincore-rpc_rust_client)
[![Crates.io](https://img.shields.io/crates/v/bitcoin_rpc_client.svg)](https://crates.io/crates/bitcoin_rpc_client)

This crate provides a Rust interface to the Bitcoin JSON-RPC API.

It is currently work-in-progress as not all RPC calls are implemented.

## Features

- Does not use macros
- Automatic retry mechanism if bitcoin-core is not yet ready
- Provides trait of all RPC methods for easy mocking (`BitcoinRpcApi`)

## Usage

Check `examples/` but basically, given a URL and the username/password for the node, you can construct a client and call the desired RPC method.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-Apache-2.0) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
