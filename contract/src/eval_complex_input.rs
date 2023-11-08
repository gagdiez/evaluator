use near_sdk::{
    env::{self, log_str},
    json_types::U64,
    near_bindgen, require,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas, Promise, PromiseError,
};

use crate::{
    constants::{NO_DEPOSIT, TGAS},
    Contract, ContractExt,
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MockTransaction {
    signer: AccountId,
    action: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MockBlock {
    author: AccountId,
    height: U64,
    transactions: Vec<MockTransaction>,
}

#[near_bindgen]
impl Contract {
    pub fn evaluate_complex_input(&mut self, contract_account_id: AccountId) -> Promise {
        self.assert_valid_account(&contract_account_id);

        let validator = format!("validator{}.testnet", self.random_u8(1));

        let args = json!({"author": validator, "height": U64(env::block_height())})
            .to_string()
            .into_bytes();

        let expected_mock_block = MockBlock {
            author: validator.parse().unwrap(),
            height: U64(env::block_height()),
            transactions: vec![
                MockTransaction {
                    signer: contract_account_id.clone(),
                    action: "FunctionCall".to_string(),
                },
                MockTransaction {
                    signer: contract_account_id.clone(),
                    action: "Transfer".to_string(),
                },
            ],
        };

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
                    .eval_ci_callback(env::predecessor_account_id(), expected_mock_block),
            )
    }

    #[private]
    pub fn eval_ci_callback(
        &mut self,
        #[callback_result] call_result: Result<MockBlock, PromiseError>,
        student_id: AccountId,
        expected_mock_block: MockBlock,
    ) -> bool {
        match call_result {
            Ok(resulted_block) => {
                log_str(&format!(
                    "Expected block to be {:#?}, received {:#?}",
                    expected_mock_block, resulted_block
                ));
                require!(resulted_block.eq(&expected_mock_block));

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
