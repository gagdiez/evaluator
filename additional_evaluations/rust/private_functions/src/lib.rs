// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::near_bindgen;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    answer: u8,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self { answer: 7 }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Get the answer (public)
    pub fn get_answer(&self) -> u8 {
        log_str(format!("The answer is {}", self.answer).as_str());
        self.answer
    }
    // Don't modify the code above this line
    // =========================================================================

    // 1. Write a internal function "add" that's only callable from the within contract itself
    // The function should accept two parameters "a"(u8) and "b"(u8) and returns the sum of the two parameters

    // Add the two numbers (internal)

    // 2. Write a public function "get_sum" that accepts two parameters "a" : u8 and "b" : u8 and returns the sum of the two parameters
    // Make use of the internal function "add" that you wrote above

    // Gets the sum of two numbers (public)

    // 3 Write a public function "set_universal_answer" that's only callable (private) from the Account ID that the contract has been deployed to
    // The function should set the answer to 42 (self.answer = 42)

    // Sets the answer to 42 (private)
}
