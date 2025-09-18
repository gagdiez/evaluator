pub use crate::constants::{BASIC_EVAL_NUMBER, CERT_CONTRACT_ACC, REGISTRATION_COST, TGAS};
use near_sdk::{
    near, store::LookupMap,
    env::{self, predecessor_account_id, random_seed},
    require, AccountId,
};

pub mod external;
pub use crate::external::*;
mod constants;
// mod eval_complex_input;
// mod eval_guestbook;
mod eval_hello;
// mod eval_xcc;

#[near(contract_state)]
pub struct Contract {
    evaluations: LookupMap<AccountId, Vec<bool>>,
    temp_u128: LookupMap<AccountId, u128>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            evaluations: LookupMap::new(b"r".to_vec()),
            temp_u128: LookupMap::new(b"t".to_vec()),
        }
    }
}

#[near]
impl Contract {
    pub fn register(&mut self) {
        let account_id = env::predecessor_account_id();

        if self.evaluations.contains_key(&account_id) {
            return;
        }

        let evaluations = vec![false; BASIC_EVAL_NUMBER];
        self.evaluations.insert(account_id, evaluations);
    }

    pub fn get_evaluations(&self, account_id: AccountId) -> Vec<bool> {
        self.evaluations.get(&account_id).unwrap().to_vec()
    }

    pub fn passed_all_exams(&self, account_id: AccountId) -> bool {
        let evaluations = self.evaluations.get(&account_id).unwrap();
        evaluations.iter().all(|&x| x)
    }

    fn assert_is_subaccount(&self, sub_account_id: &AccountId) {
        let parent_id: AccountId = self.get_parent_account(&sub_account_id);

        // Only parent accounts can evaluate sub-accounts
        require!(
            parent_id == predecessor_account_id(),
            format!("Only {} can evaluate {}", parent_id, sub_account_id)
        );
    }

    fn get_parent_account(&self, sub_account_id: &AccountId) -> AccountId {
        sub_account_id
            .to_string()
            .split(".")
            .skip(1)
            .collect::<Vec<&str>>()
            .join(".")
            .parse()
            .unwrap()
    }

    pub fn check_account_registered(&self, account_id: &AccountId) -> bool {
        self.evaluations.contains_key(account_id)
    }

    pub fn get_progress(&self, account_id: AccountId) -> Vec<bool> {
        let evaluations = self.evaluations.get(&account_id).unwrap();
        evaluations.clone()
    }

    fn random_string(&self, seed: u8) -> String {
        let get_array: Vec<u8> = random_seed();
        String::from_utf8_lossy(&get_array).to_string() + &seed.to_string()
    }

    fn random_u128(&self, seed: u8) -> u128 {
        let random_seed = random_seed();
        self.as_u128(random_seed.get(..16).unwrap()) + seed as u128
    }

    fn random_u8(&self, seed: usize) -> u8 {
        env::random_seed()[seed]
    }

    fn as_u128(&self, arr: &[u8]) -> u128 {
        ((arr[0] as u128) << 0)
            + ((arr[1] as u128) << 8)
            + ((arr[2] as u128) << 16)
            + ((arr[3] as u128) << 24)
            + ((arr[4] as u128) << 32)
            + ((arr[5] as u128) << 40)
            + ((arr[6] as u128) << 48)
            + ((arr[7] as u128) << 56)
            + ((arr[8] as u128) << 64)
            + ((arr[9] as u128) << 72)
            + ((arr[10] as u128) << 80)
            + ((arr[11] as u128) << 88)
            + ((arr[12] as u128) << 96)
            + ((arr[13] as u128) << 104)
            + ((arr[14] as u128) << 112)
            + ((arr[15] as u128) << 120)
    }
}

// UNIT TESTS
// Note: #[private] macro doesn't expand in unit tests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .is_view(is_view)
            .current_account_id("contract.testnet".parse().unwrap());
        builder
    }

    #[test]
    fn test_evaluating_sub_account() {
        let mut context = get_context(false);
        let mut contract = Contract::default();

        testing_env!(context
            .predecessor_account_id("someone.testnet".parse().unwrap())
            .build());

        contract.register();
        contract.assert_is_subaccount(&"hello_near.someone.testnet".parse().unwrap());
    }

    #[test]
    fn test_random_string() {
        let mut context = get_context(false);
        let contract = Contract::default();

        testing_env!(context
            .predecessor_account_id("someone.testnet".parse().unwrap())
            .build());

        assert!(contract.random_string(1) != contract.random_string(2));
    }
    #[test]
    fn test_get_parent_account() {
        let mut context = get_context(false);
        let contract = Contract::default();

        testing_env!(context
            .predecessor_account_id("someone.testnet".parse().unwrap())
            .build());

        let expected: AccountId = "someone.testnet".parse().unwrap();
        assert_eq!(
            contract.get_parent_account(&"hello_near.someone.testnet".parse().unwrap()),
            expected
        );
    }
    #[test]
    fn test_check_account_registered() {
        let mut context = get_context(false);
        let mut contract = Contract::default();

        testing_env!(context
            .predecessor_account_id("someone.testnet".parse().unwrap())
            .build());

        contract.register();

        assert!(contract.check_account_registered(&"someone.testnet".parse().unwrap()));
        assert_eq!(
            contract.check_account_registered(&"hello_near.someone.testnet".parse().unwrap()),
            false
        );
    }
}
