use std::cell::RefCell;

use candid::Principal;
use ic_cdk::export_candid;
use ic_ledger_types::AccountIdentifier;
use prelude::*;
use types::*;

mod dip20;
mod guard;
mod lifecycle;
mod prelude;
mod query;
mod types;
mod update;
mod util;

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default()
}

export_candid!();
