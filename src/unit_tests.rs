#[cfg(test)]
mod tests {
    use crate::contract::Contract;
    use near_sdk::json_types::U128;
    use near_sdk::AccountId;
    use near_sdk::CryptoHash;

    #[test]
    #[should_panic]
    fn new_method_unimplemented() {
        // new is currently unimplemented and should panic
        let owner: AccountId = "owner.testnet".parse().unwrap();
        let fee = U128(0);
        let code_hash = CryptoHash::default();
        let _ = Contract::new(owner, fee, code_hash);
    }
}
