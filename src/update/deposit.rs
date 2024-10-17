use candid::Principal;
use ic_cdk::{api::id, caller, println, update};
use ic_ledger_types::{
    account_balance, transfer, AccountBalanceArgs, AccountIdentifier, Memo, TransferArgs,
    DEFAULT_FEE, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

use crate::{
    dip20::DIP20,
    prelude::{
        Error::{BalanceLow, TransferFailed},
        Response,
    },
    util::principal_to_subaccount,
    RUNTIME_STATE,
};

#[update]
pub async fn deposit(token_canister_id: Principal) -> Response<()> {
    RUNTIME_STATE
        .with(|state| {
            deposit_impl(
                token_canister_id,
                state.borrow().ledger.unwrap_or(MAINNET_LEDGER_CANISTER_ID),
            )
        })
        .await
}

async fn deposit_impl(token_canister_id: Principal, ledger_canister_id: Principal) -> Response<()> {
    let canister_id = id();
    let caller = caller();

    println!("Depositing ICP from account {}", caller);
    match token_canister_id {
        token_canister_id if token_canister_id == ledger_canister_id => {
            let account = AccountIdentifier::new(&canister_id, &principal_to_subaccount(&caller));

            let balance = account_balance(ledger_canister_id, AccountBalanceArgs { account })
                .await
                .map_err(|_| TransferFailed)?;

            if balance < DEFAULT_FEE {
                return Err(BalanceLow);
            }

            let amount = balance - DEFAULT_FEE;

            let args = TransferArgs {
                memo: Memo(0),
                amount,
                fee: DEFAULT_FEE,
                from_subaccount: Some(principal_to_subaccount(&caller)),
                to: AccountIdentifier::new(&canister_id, &DEFAULT_SUBACCOUNT),
                created_at_time: None,
            };

            transfer(ledger_canister_id, args)
                .await
                .map_err(|_| TransferFailed)?
                .map_err(|_| TransferFailed)?;
        }
        token_canister_id => {
            let token = DIP20::new(token_canister_id);

            let dip_fee = token.metadata().await.fee;

            let allowance = token.allowance(caller, canister_id).await;

            let available = allowance - dip_fee;

            // TODO: Check if the caller has enough balance

            token
                .transfer_from(caller, canister_id, available.to_owned())
                .await
                .map_err(|_| TransferFailed)?;
        }
    };

    Ok(())
}

#[update(name = "getDepositAddress")]
pub fn get_deposit_address() -> AccountIdentifier {
    let canister_id = id();
    let subaccount = principal_to_subaccount(&caller());

    AccountIdentifier::new(&canister_id, &subaccount)
}
