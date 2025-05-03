set -e 

RUSTFLAGS=-Awarnings cargo build -p ic-lightclient-oc-agent

dfx start --clean --background
dfx deploy canister --specified-id uxrrr-q7777-77774-qaaaq-cai

initial_block_hash=$(dfx canister call canister get_latest_block_hash)

cargo run ic-lightclient-oc-agent &
sleep 30

new_block_hash=$(dfx canister call canister get_latest_block_hash)

jobs -p | xargs kill -9
dfx stop

if [[ $initial_block_hash == $new_block_hash ]]; then
    exit 1
fi