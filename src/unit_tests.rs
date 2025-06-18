#[cfg(test)]
mod tests {
    use crate::contract::Contract;
    use near_sdk::env;
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::AccountId;
    use near_sdk::CryptoHash;
    use near_sdk::NearToken;

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: AccountId, amount: NearToken) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }

    #[test]
    fn new_initializes_fields() {
        let owner: AccountId = accounts(0);
        set_context(owner.clone(), NearToken::from_near(0));
        let fee = U128(123);
        let code_hash = CryptoHash::default();
        let contract = Contract::new(owner.clone(), fee, code_hash);
        assert_eq!(contract.owner_id(), &owner);
        assert_eq!(contract.vault_minting_fee(), 123);
        assert_eq!(contract.vault_code_hash(), code_hash);
        assert_eq!(contract.vault_count(), 0);
    }

    #[test]
    #[should_panic(expected = "Contract is already initialized")]
    fn new_reinitialization_panics() {
        let owner: AccountId = accounts(1);
        set_context(owner.clone(), NearToken::from_near(0));
        let fee = U128(456);
        let code_hash = CryptoHash::default();
        let contract1 = Contract::new(owner.clone(), fee, code_hash);
        // simulate state persisted by writing contract1 to storage
        env::state_write(&contract1);
        // second init on existing state should panic
        let _ = Contract::new(owner, fee, code_hash);
    }
}
