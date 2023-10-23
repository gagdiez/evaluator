use near_sdk::{
    env, json_types::U128, near_bindgen, require, AccountId, Gas, Promise, PromiseError,
};

use crate::{
    constants::{NO_ARGS, NO_DEPOSIT, TGAS},
    Contract, ContractExt,
};

#[near_bindgen]
impl Contract {
    // Provides a temporary seed to the students sub-account (contract)
    pub fn provide_temp_seed(&mut self) -> U128 {
        require!(
            self.check_account_registered(&env::signer_account_id()),
            "This account is not registered"
        );
        require!(
            env::signer_account_id() != env::predecessor_account_id(),
            format!(
                "This function shall be called via XCC from a sub account such as valid.{}",
                env::signer_account_id()
            )
        );
        // TODO: Use randomness from the blockchain
        let seed = U128(1234567890u128);

        self.temp_seeds
            .insert(&env::predecessor_account_id(), &seed.0);

        return seed;
    }
    // Evaluate XCC from the students contract to verify the temp seed
    pub fn evaluate_xcc(&mut self) -> Promise {
        Promise::new(env::predecessor_account_id())
            .function_call(
                "get_current_seed".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas(15 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_current_seed(),
            )
    }
    // Evaluate current seed
    #[private]
    pub fn evaluate_current_seed(
        &mut self,
        #[callback_result] call_result: Result<U128, PromiseError>,
    ) {
        match call_result {
            Ok(current_seed) => {
                let expected_seed = &self.temp_seeds.get(&env::predecessor_account_id()).unwrap();

                require!(
                    current_seed.0 == *expected_seed,
                    format!(
                        "Expected seed to be {}, not {}",
                        expected_seed, current_seed.0
                    )
                );
                self.temp_seeds.remove(&env::predecessor_account_id());
            }
            Err(err) => {
                self.temp_seeds.remove(&env::predecessor_account_id());
                require!(false, format!("{:#?}", err));
            }
        }
    }
}
