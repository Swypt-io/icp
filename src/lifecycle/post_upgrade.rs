use ic_cdk::{post_upgrade, storage};

use crate::{types::RuntimeState, RUNTIME_STATE};

#[post_upgrade]
fn post_upgrade() {
    let (last_state,): (RuntimeState,) = storage::stable_restore().unwrap();

    RUNTIME_STATE.with(|state| *state.borrow_mut() = last_state);
}
