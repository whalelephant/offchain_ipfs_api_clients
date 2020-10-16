## Rust-based clients for interacting with Offchain::ipfs

This is a repository to demonstrate how to interact with [Offchain::ipfs](https://rs-ipfs.github.io/offchain-ipfs-manual/), specifically the showcase pallet. 
Two api clients are used [subxt](https://github.com/paritytech/substrate-subxt) and [substrate-api-client](https://github.com/scs/substrate-api-client)


### Getting started
Make sure you have the *Offchain::IPFS* node running with the default **--ws-port 9944**.

```sh
cargo build 
./target/debug/subxt-ipfs #for the subxt client demo
./target/debug/sub-api-ipfs #for the substrate-api-client demo
```

