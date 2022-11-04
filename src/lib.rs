mod interface;

use interface::ft::ft;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    log, near_bindgen, Gas,
};

pub const NEAR: u128 = 10u128.pow(24);

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {}
    }

    pub fn withdraw_call(msg: String) {
        log!("prepaid gas: {}", env::prepaid_gas().0);
        log!("replenisher withdraw call: msg: {}", msg);
        let receiver_id = env::predecessor_account_id();
        let amount = U128(NEAR);
        ft::ext("wrap.testnet".parse().unwrap())
            .with_attached_deposit(1)
            .with_static_gas(Gas::ONE_TERA * 260)
            .ft_transfer_call(receiver_id, amount, msg);
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new()
    }
}
