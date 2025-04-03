cargo build --release --target wasm32-unknown-unknown --package ic-lightclient-canister
candid-extractor target/wasm32-unknown-unknown/release/ic_lightclient_canister.wasm > src/canister/canister.did

dfx build --check canister