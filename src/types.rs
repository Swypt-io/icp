use candid::{CandidType, Deserialize, Nat, Principal};

#[derive(Default, CandidType, Deserialize)]
pub struct RuntimeState {
    pub owner: Option<Principal>,
    pub ledger: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct WithdrawArgs {
    pub token: Principal,
    pub amount: Nat,
    pub to: Principal,
}
