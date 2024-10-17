use ic_cdk::caller;

use crate::RUNTIME_STATE;

pub fn only_owner() -> Result<(), String> {
    if RUNTIME_STATE.with(|state| state.borrow().owner == Some(caller())) {
        Ok(())
    } else {
        Err("Only the owner can withdraw tokens".to_string())
    }
}
