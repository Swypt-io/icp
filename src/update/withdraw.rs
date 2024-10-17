use candid::Principal;
use ic_cdk::{api::id, caller, println, update};
use ic_ledger_types::{
    transfer, AccountIdentifier, Memo, Tokens, TransferArgs, DEFAULT_FEE, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};

use crate::{
    dip20::DIP20, guard::only_owner, prelude::Error::*, types::WithdrawArgs, Response,
    RUNTIME_STATE,
};

#[update(guard = "only_owner")]
pub async fn withdraw(args: WithdrawArgs) -> Response<()> {
    RUNTIME_STATE
        .with(|state| {
            withdraw_impl(
                args,
                state.borrow().ledger.unwrap_or(MAINNET_LEDGER_CANISTER_ID),
            )
        })
        .await
}

async fn withdraw_impl(args: WithdrawArgs, ledger_canister_id: Principal) -> Response<()> {
    let id = id();
    let caller = caller();

    println!("Withdrawing {} tokens to account {}", args.amount, caller);
    match args.token {
        token if token == ledger_canister_id => {
            let amount = Tokens::from_e8s((args.amount).0.try_into().map_err(|_| TransferFailed)?);

            if amount < DEFAULT_FEE {
                return Err(BalanceLow);
            }

            let args = TransferArgs {
                memo: Memo(0),
                amount,
                fee: DEFAULT_FEE,
                from_subaccount: None,
                to: AccountIdentifier::new(&args.to, &DEFAULT_SUBACCOUNT),
                created_at_time: None,
            };

            transfer(ledger_canister_id, args)
                .await
                .map_err(|_| TransferFailed)?
                .map_err(|_| TransferFailed)?;

            println!("Deposited {} ICP in account {}", amount, caller);
        }
        token => {
            let token = DIP20::new(token);

            let dip_fee = token.metadata().await.fee;

            let allowance = token.allowance(caller, id).await;

            let available = allowance - dip_fee.to_owned();

            if available < args.amount {
                return Err(BalanceLow);
            }

            token
                .transfer(caller, args.amount.to_owned() + dip_fee)
                .await
                .map_err(|_| TransferFailed)?;
        }
    }
    Ok(())
}
