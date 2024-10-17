set -e
dfx stop && dfx start --background --clean

# Deploy local ledger
dfx identity new minter --storage-mode=plaintext --force
dfx identity use minter
export MINTER_ACCOUNT_ID=$(dfx ledger account-id)

dfx identity use default
export LEDGER_ACCOUNT_ID=$(dfx ledger account-id)

# User private api for install
rm icp/ledger/ledger.did || true
cp icp/ledger/ledger.private.did icp/ledger/ledger.did

dfx deploy ledger --argument '(record  {
    minting_account = "'${MINTER_ACCOUNT_ID}'";
    initial_values = vec { record { "'${LEDGER_ACCOUNT_ID}'"; record { e8s=100_000_000_000 } }; };
    send_whitelist = vec {}
    })'
    
export LEDGER_CANISTER_ID=$(dfx canister id ledger)

# Deploy token DIP Tokens

dfx canister create SwyptDIP20
dfx build SwyptDIP20

export ROOT_PRINCIPAL="principal \"$(dfx identity get-principal)\""
echo $ROOT_PRINCIPAL
dfx canister install SwyptDIP20 --argument="(\"https://dogbreedslist.com/wp-content/uploads/2019/08/Are-Golden-Retrievers-easy-to-train.png\", \"Swypt Coin\", \"SWYPT\", 8, 100_000_000_000, $ROOT_PRINCIPAL, 10_000)"

# Set Fees
dfx canister call SwyptDIP20 setFeeTo "($ROOT_PRINCIPAL)"
dfx canister call SwyptDIP20 setFee "(420)"

# Deploy Internet Identity 
II_FETCH_ROOT_KEY=1 dfx deploy internet_identity --no-wallet --argument '(null)'

# Install Backend

dfx deploy icp --argument "(opt principal \"$LEDGER_CANISTER_ID\")"

dfx generate icp
dfx build icp
dfx build SwyptDIP20
dfx generate icp
dfx generate SwyptDIP20
dfx generate ledger
