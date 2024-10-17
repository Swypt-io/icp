use candid::Principal;
use ic_cdk::{caller, query};

#[query]
pub fn whoami() -> Principal {
    caller()
}
