use ic_cdk::{pre_upgrade, storage};

use crate::RUNTIME_STATE;

#[pre_upgrade]
fn pre_upgrade() {
    RUNTIME_STATE.with(|state| storage::stable_save((state,)).unwrap());
}
