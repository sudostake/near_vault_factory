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

// Manual Default impl required by the near macro (for migrations).
impl Default for Contract {
    fn default() -> Self {
        near_sdk::env::panic_str("Contract is not initialized");
    }
}

// Public getters for testing purposes
#[cfg(test)]
impl Contract {
    pub fn owner_id(&self) -> &AccountId {
        &self.owner_id
    }

    pub fn vault_minting_fee(&self) -> u128 {
        self.vault_minting_fee
    }

    pub fn vault_code_hash(&self) -> CryptoHash {
        self.vault_code_hash
    }

    pub fn vault_count(&self) -> u64 {
        self.vault_count
    }
}

// Implement the contract structure
#[allow(dead_code, unused_variables)]
#[near]
impl Contract {
    /// Initializes the contract.
    #[init]
    pub fn new(
        owner_id: AccountId,
        vault_minting_fee: U128,
        initial_vault_code_hash: CryptoHash,
    ) -> Self {
        assert!(
            !near_sdk::env::state_exists(),
            "Contract is already initialized"
        );
        Self {
            owner_id,
            vault_minting_fee: vault_minting_fee.0,
            vault_code_hash: initial_vault_code_hash,
            vault_count: 0,
        }
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
    /// Returns the contract owner.
    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// Returns the vault minting fee.
    pub fn get_vault_minting_fee(&self) -> U128 {
        U128(self.vault_minting_fee)
    }

    /// Returns the vault code hash.
    pub fn get_vault_code_hash(&self) -> CryptoHash {
        self.vault_code_hash
    }

    /// Returns the number of vaults minted so far.
    pub fn get_vault_count(&self) -> u64 {
        self.vault_count
    }
}
