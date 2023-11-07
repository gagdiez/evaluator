use near_sdk::{
    env::{self, log_str},
    near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas, Promise, PromiseError,
};

use crate::{
    constants::{NO_DEPOSIT, TGAS},
    Contract, ContractExt,
};
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MockTransaction {
    signer: AccountId,
    action: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MockBlock {
    author: AccountId,
    height: u8,
    transactions: Vec<MockTransaction>,
}

#[near_bindgen]
impl Contract {
    pub fn evaluate_complex_input(&mut self, contract_account_id: AccountId) -> Promise {
        self.assert_valid_account(&contract_account_id);

        let validator = format!("validator{}.testnet", self.random_u8(1));
        let height = self.random_u8(2);

        let mock_tx = MockTransaction {
            signer: env::predecessor_account_id(),
            action: "FunctionCall".to_string(),
        };

        let args = json!({"author": validator, "height": height, "transactions": [mock_tx]})
            .to_string()
            .into_bytes();

        Promise::new(contract_account_id.clone())
            .function_call(
                "provide_output".to_string(),
                args,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .eval_ci_callback(env::predecessor_account_id()),
            )
    }

    #[private]
    pub fn eval_ci_callback(
        &mut self,
        #[callback_result] call_result: Result<MockBlock, PromiseError>,
        student_id: AccountId,
    ) -> bool {
        match call_result {
            Ok(_expected) => {
                let mut evaluations = self.evaluations.get(&student_id).unwrap();

                evaluations[3] = true;

                self.evaluations.insert(&student_id, &evaluations);

                true
            }
            Err(err) => {
                log_str(&format!("{:#?}", err));
                false
            }
        }
    }
}
