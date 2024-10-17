use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::call;

#[derive(Deserialize, CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other,
}
pub type TxReceipt = Result<Nat, TxError>;

#[allow(non_snake_case)]
#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct Metadata {
    pub logo: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub totalSupply: Nat,
    pub owner: Principal,
    pub fee: Nat,
}

pub struct DIP20 {
    principal: Principal,
}

impl DIP20 {
    pub fn new(principal: Principal) -> Self {
        Self { principal }
    }

    pub async fn allowance(&self, owner: Principal, spender: Principal) -> Nat {
        let result: Result<(Nat,), _> = call(self.principal, "allowance", (owner, spender)).await;

        result.unwrap().0
    }

    pub async fn metadata(&self) -> Metadata {
        let result: Result<(Metadata,), _> = call(self.principal, "getMetadata", ()).await;

        result.unwrap().0
    }

    pub async fn transfer(&self, to: Principal, amount: Nat) -> TxReceipt {
        let result: Result<(TxReceipt,), _> = call(self.principal, "transfer", (to, amount)).await;

        result.unwrap().0
    }

    pub async fn transfer_from(&self, from: Principal, to: Principal, amount: Nat) -> TxReceipt {
        let result: Result<(TxReceipt,), _> =
            call(self.principal, "transferFrom", (from, to, amount)).await;

        result.unwrap().0
    }
}
