use candid::CandidType;

#[derive(CandidType)]
pub enum Error {
    BalanceLow,
    TransferFailed,
}

pub type Response<T> = Result<T, Error>;
