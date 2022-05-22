use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
const ONE_TENTH_NEAR: u128 = 100_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Contract {
  counter: i32
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Contract { counter: 0 }
  }

  #[payable]
  pub fn functional_function(&mut self) {
    if env::attached_deposit() >= ONE_TENTH_NEAR {
      self.counter += 1;
      env::log("This was a call of a functional_function".as_ref())
    } else {
      env::panic("No beautiful message".as_ref())
    }
  }
}

#[cfg(test)]
mod tests {
  use near_sdk::json_types::ValidAccountId;
  use near_sdk::MockedBlockchain;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::testing_env;
  use crate::Contract;

  #[test]
  fn test() {
    let test_id = ValidAccountId::try_from("foo.testnet".to_string()).unwrap();
    let test_context = VMContextBuilder::new()
      .current_account_id(test_id)
      .attached_deposit(100_000_000_000_000_000_000_000)
      .build();
    testing_env!(test_context);

    let mut contract = Contract { counter: 1 };
    contract.functional_function()
  }
}
