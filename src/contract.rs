// Find all our documentation at https://docs.near.org
use near_sdk::json_types::U128;
use near_sdk::{near, AccountId, CryptoHash};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    owner_id: AccountId,
    vault_minting_fee: u128,
    vault_code_hash: CryptoHash,
    vault_count: u64,
}

// Implement the contract structure
#[allow(dead_code, unused_variables)]
#[near]
impl Contract {
    /// Initializes the contract. Only the deployer of the contract can call it.
    #[init]
    pub fn new(
        owner_id: AccountId,
        vault_minting_fee: U128,
        initial_vault_code_hash: CryptoHash,
    ) -> Self {
        // TODO
        unimplemented!()
    }

    /// Mints a new vault instance under a sequential subaccount. The caller must attach the vault minting fee.
    #[payable]
    pub fn mint_vault(&mut self) {
        unimplemented!()
    }

    /// Updates the vault minting fee. Only the owner can call this.
    pub fn update_vault_minting_fee(&mut self, vault_minting_fee: U128) {
        unimplemented!()
    }

    /// Updates the vault code hash. Only the owner can call this.
    pub fn update_vault_code_hash(&mut self, vault_code_hash: CryptoHash) {
        unimplemented!()
    }

    /// Withdraws the available contract balance (minus storage cost) to the given recipient.
    pub fn withdraw_available_balance(&mut self, recipient: AccountId) {
        unimplemented!()
    }

    /// Transfers ownership of the factory to a new owner.
    pub fn transfer_ownership(&mut self, new_owner_id: AccountId) {
        unimplemented!()
    }
}
