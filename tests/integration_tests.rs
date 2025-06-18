// Integration tests for the vault factory contract (testing `new` method) using near-workspaces.
use near_sdk::json_types::U128;
use near_sdk::CryptoHash;
use near_workspaces::{Contract, ContractState, Worker};
use near_workspaces::network::Sandbox;

// Utility to deploy the vault contract as a top-level account and return its code hash.
async fn deploy_vault_as_global_contract(
    worker: &Worker<Sandbox>,
) -> Result<CryptoHash, Box<dyn std::error::Error>> {
    let wasm = std::fs::read("tests/res/vault.wasm")?;
    let vault = worker.dev_deploy_tla(&wasm).await?;
    let details = worker.view_account(vault.id()).await?;
    let hash = match details.contract_state {
        ContractState::LocalHash(h) | ContractState::GlobalHash(h) => h.0,
        state => panic!("unexpected vault contract state: {:?}", state),
    };
    Ok(hash)
}

// TODO add a utility function called init_factory.
// It should deploy the factory contract and call `new` with the given parameters.

#[tokio::test]
async fn integration_new_initialization() -> Result<(), Box<dyn std::error::Error>> {
    // Launch sandbox and compile + deploy the contract on the fly
    let worker = near_workspaces::sandbox().await?;
    let wasm = near_workspaces::compile_project("./").await?;
    let contract = worker.dev_deploy(&wasm).await?;

    let code_hash = deploy_vault_as_global_contract(&worker).await?;

    // Deployer account calls `new`
    let owner = worker.dev_create_account().await?;
    let fee = U128(123);

    // Initialize the contract
    let outcome = owner
        .call(contract.id(), "new")
        .args_json((owner.id(), fee, code_hash))
        .transact()
        .await?;
    assert!(outcome.is_success());

    // Verify via view calls
    let got_owner: String = contract.view("get_owner_id").await?.json()?;
    assert_eq!(got_owner, owner.id().to_string());

    let got_fee: U128 = contract.view("get_vault_minting_fee").await?.json()?;
    assert_eq!(got_fee.0, fee.0);

    let got_hash: CryptoHash = contract.view("get_vault_code_hash").await?.json()?;
    assert_eq!(got_hash, code_hash);

    let got_count: u64 = contract.view("get_vault_count").await?.json()?;
    assert_eq!(got_count, 0);

    Ok(())
}
