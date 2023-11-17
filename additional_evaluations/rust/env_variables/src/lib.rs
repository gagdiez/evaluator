// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId};
use near_sdk::{log, near_bindgen};
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn check_predecessor(&self) -> bool {
        // check that the predecessor is "bob.test
        log!(format!(
            "predecessor_account_id: {}",
            env::predecessor_account_id()
        ));

        env::predecessor_account_id() == AccountId::new_unchecked("allowed.test.near".to_string())
    }

    // #[payable]
    // pub fn get_attached_deposit(&self) -> Bool {
    //     // Assert that the attached deposit is greater than 3 NEAR
    // }

    pub fn get_current_block_timestamp(&self) -> u64 {
        env::block_timestamp()
    }
}
