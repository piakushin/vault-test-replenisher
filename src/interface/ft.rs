use near_sdk::{ext_contract, json_types::U128, AccountId};

#[ext_contract(ft)]
trait FungibleToken {
    fn ft_transfer_call(receiver_id: AccountId, amount: U128, msg: String);
}
