use near_sdk::{
    ext_contract, near, AccountId
};

#[near(serializers = [json, borsh])]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

// Validator interface, for cross-contract calls
#[ext_contract(hello_near)]
trait HelloNear {
    fn get_greeting(&self) -> String;
    fn set_greeting(&self, greeting: String);
}

#[ext_contract(guestbook)]
trait GuestBook {
    fn add_message(&mut self, text: String);
    fn get_messages(&self) -> Vec<PostedMessage>;
}
