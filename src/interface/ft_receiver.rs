use near_contract_standards::{
    fungible_token::receiver::FungibleTokenReceiver, non_fungible_token::TokenId,
};
use near_sdk::{
    ext_contract,
    json_types::U128,
    log, near_bindgen,
    serde::{Deserialize, Serialize},
    serde_json::from_str,
    AccountId, Gas, Promise, PromiseOrValue,
};

use crate::{Contract, ContractExt};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        log!(
            "ft_on_transfer: sender_id: {sender_id} amount: {} msg: {msg}",
            amount.0,
        );
        let req: Request = from_str(&msg).unwrap();

        Promise::new(req.vault).function_call(
            "add_replenishment_callback".to_owned(),
            req.args.into_bytes(),
            1,
            Gas::ONE_TERA * 100,
        );

        PromiseOrValue::Value(U128(0))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    vault: AccountId,
    args: String,
}

#[ext_contract(vault)]
trait Vault {
    fn add_replenishment_callback(
        &mut self,
        nft_contract_id: AccountId,
        nft_id: TokenId,
        callback: String,
        args: String,
    );
}
