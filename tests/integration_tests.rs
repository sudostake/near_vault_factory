// Integration tests for the vault factory contract (testing `new` method) using near-workspaces.
use near_sdk::json_types::U128;
use near_sdk::CryptoHash;
use near_workspaces::sandbox;

#[tokio::test]
async fn integration_new_initialization() -> Result<(), Box<dyn std::error::Error>> {
    // Launch sandbox and compile + deploy the contract on the fly
    let worker = sandbox().await?;
    let wasm = near_workspaces::compile_project("./").await?;
    let contract = worker.dev_deploy(&wasm).await?;

    // Deployer account calls `new`
    let owner = worker.dev_create_account().await?;
    let fee = U128(123);
    let code_hash = CryptoHash::default();

    // Initialize the contract
    let outcome = owner
        .call(&contract.id(), "new")
        .args_json((owner.id(), fee, code_hash.clone()))
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
