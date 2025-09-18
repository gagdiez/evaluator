use near_sdk::{
    env::{self, log_str},
    near,
    serde_json::json,
    AccountId, Gas, Promise, PromiseError,
};

pub use crate::constants::{NO_ARGS, NO_DEPOSIT};
use crate::{Contract, ContractExt};

#[near]
impl Contract {

    pub fn evaluate_hello_near_subaccount(&mut self, contract_account_id: AccountId) -> Promise {
        self.assert_valid_account(&contract_account_id);
        self.evaluate_hello_near(contract_account_id)
    }

    pub fn evaluate_hello_near(&mut self, contract_account_id: AccountId) -> Promise {
        let random_string: String = self.random_string(1);

        let args = json!({ "greeting": random_string })
            .to_string()
            .into_bytes();

        Promise::new(contract_account_id.clone())
            .function_call("set_greeting".to_string(), args, NO_DEPOSIT, Gas::from_tgas(15))
            .function_call(
                "get_greeting".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas::from_tgas(5),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(5))
                    .evaluate_hello_near_callback(env::predecessor_account_id(), random_string),
            )
    }
    // Hello Near Evaluation Callback
    #[private]
    pub fn evaluate_hello_near_callback(
        &mut self,
        #[callback_result] call_result: Result<String, PromiseError>,
        student_id: AccountId,
        random_string: String,
    ) -> bool {
        let mut passed = false;
        match call_result {
            Ok(greeting) => {
                log_str(&format!(
                    "Expected greeting to be {}, not {}",
                    random_string, greeting
                ));
                if greeting == random_string {
                    passed = true;
                    let mut student_evals = self.evaluations.get(&student_id).unwrap().clone();
                    student_evals[0] = true;
                    self.evaluations.set(student_id, Option::Some(student_evals));
                };
            }
            Err(err) => {
                log_str(&format!("{:#?}", err));
            }
        }
        passed
    }
}
