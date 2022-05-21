use near_sdk::near_bindgen;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Contract;

#[near_bindgen]
impl Contract {

}

#[cfg(test)]
mod tests {
  use near_sdk::json_types::ValidAccountId;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::testing_env;
  use near_sdk::MockedBlockchain;

  #[test]
  fn test() {
    let test_id = ValidAccountId::try_from("foo.testnet".to_string()).unwrap();
    let test_context = VMContextBuilder::new()
      .current_account_id(test_id)
      .build();
    testing_env!(test_context);
  }
}
