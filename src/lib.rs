// NEAR Vault Factory contract.
// See documentation: https://docs.near.org

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{near, AccountId};

/// Contract state for the Vault Factory.
#[near(contract_state)]
pub struct VaultFactory {
    owner_id: AccountId,
    vault_minting_fee: U128,
    /// Ordered list of unique vault code hashes.
    vault_code_hashes: Vec<Vec<u8>>,
    /// Latest code hash version.
    latest_code_hash_version: u64,
}

#[near]
impl VaultFactory {
    /// Initializes the factory. Can only be called once.
    #[init]
    pub fn new(
        owner_id: AccountId,
        vault_minting_fee: U128,
        initial_vault_code_hash: Vec<u8>,
    ) -> Self {
        todo!()
    }

    /// Updates the vault minting fee. Only callable by the owner.
    pub fn update_vault_minting_fee(&mut self, vault_minting_fee: U128) {
        todo!()
    }

    /// Adds a new vault code hash version. Only callable by the owner.
    /// Ensures the code hash is unique across existing versions.
    pub fn update_vault_code_hash(&mut self, vault_code_hash: Vec<u8>) {
        todo!()
    }

    /// Returns the vault code hash for the given version.
    pub fn get_vault_code_hash(&self, version: u64) -> Option<Vec<u8>> {
        todo!()
    }

    /// Returns the latest vault code hash.
    pub fn get_latest_vault_code_hash(&self) -> Vec<u8> {
        todo!()
    }

    /// Mints a new vault by attaching the required fee.
    #[payable]
    pub fn mint_vault(&mut self) -> AccountId {
        todo!()
    }

    /// Transfers factory ownership to a new account. Only callable by the owner.
    pub fn transfer_ownership(&mut self, new_owner_id: AccountId) {
        todo!()
    }

    /// Withdraws available proceeds (NEAR tokens) to the given account. Only callable by the owner.
    pub fn withdraw_proceeds(&mut self, target_account_id: AccountId, amount: U128) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_new() {
        let owner: AccountId = "owner.testnet".parse().unwrap();
        let code_hash = vec![];
        let _ = VaultFactory::new(owner, U128(0), code_hash);
    }
}
