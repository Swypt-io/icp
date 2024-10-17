use candid::Principal;
use ic_cdk::{caller, init, setup};

use crate::RUNTIME_STATE;

#[init]
fn init(ledger_canister_id: Option<Principal>) {
    setup();

    RUNTIME_STATE.with(|state| {
        state.borrow_mut().ledger = ledger_canister_id;
        state.borrow_mut().owner = Some(caller());
    });
}
