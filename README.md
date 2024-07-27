
# Turbin3-Prereq Solana development (Rust)

Within this project, we create a secret key and public key, request for an airdrop from solana devnet, made a transfer to another wallet, emptied a wallet, interacted with the WBA anchor IDL, and created program derived addresses.


## Acknowledgements

 - [Turbin3](https://turbin3.com)
 - [WBA](https://https://solana.web3builders.dev/)
 - [Solana cookbook](https://solanacookbook.com)



## Documentation

The files created in this projects include
```bash
lib.rs
mod.rs
wba_prereq.rs
```

The functions created in this the lib.rs which are functional include
```bash
keygen- generates a new Keypair 
airdrop- claim token airdrops
transfer.ts - initiate a transfer to another wallet
consume_idl -  interact with the anchor IDL program that WBA has created on the devnet

 ```


## Deployment

To initiate a new project, use cargo. Ensure you have the latest version of Rust installed, which should come with cargo, and solana CLI. If required, check your env. variables
```bash
  cargo init --<name of project>
```
To run a function, click "run test" or execute 
```bash
    cargo test <function-name>

```

After running the a transaction, the transaction URL is printed

Explorer URL for transfer: https://explorer.solana.com/tx/46YEajRXtta8B9NKD3NYvYehQX1y81iLGWwFYZYYBcMgGtPVgJSFzsGJ52a86R9bSrteZ2g9zsk4xeetB9cvHpCw/?cluster=devnet

Explorer URL for IDL:  https://explorer.solana.com/tx/2LYrdL6c7bPmJjipAQvdQJxgRoyjPcGBoZhhQK9hXM64pgQ3ET6dkeushyGDA3uUvNW7oKcV9G8n8GTf62b1nMvk/?cluster=devnet



## Installation
Install Rust, Rustup and Cargo

Open up a terminal window (for MacOS) or command prompt (for Windows) and paste this command.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Install Solana Tool Suite using brew 
```bash
  brew install solana
  solana --version
```
You can refer to the cookbook for more guide on installing the solana CLI.

Cheers and don't forget to give me a follow🙂

