# Basic Contract on Near using Rust 


### Commands to run.

1. Generate Wasm from Contract `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

2. Copy Wasm file to main folder `cp target/wasm32-unknown-unknown/release/basic_contract.wasm wasmfile/`

3. Deploy to test net `near deploy --wasmFile wasmfile/basic_contract.wasm --accountId adeel.testnet`

4. Initialize Call `near call  adeel.testnet  new '{}' --accountId adeel.testnet`

5. Change Method `near call  adeel.testnet  update_score '{"score": 123, "name": "Adeel"}' --accountId adeel.testnet`

6. View Method (working on it)