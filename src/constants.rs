use near_sdk::NearToken;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: NearToken = NearToken::from_yoctonear(0);
pub const NO_ARGS: Vec<u8> = vec![];
pub const REGISTRATION_COST: NearToken = NearToken::from_yoctonear(0);
pub const BASIC_EVAL_NUMBER: usize = 4;
pub const CERT_CONTRACT_ACC: &str = "certificate_issuer.test.near";