#[tokio::main]
#[cfg(feature = "fireblocks")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ethers::{prelude::*, utils::parse_ether};

    // Connect over websockets
    let provider = Provider::new(Ws::connect("ws://localhost:8545").await?);

    // Define the configuration for working with Fireblocks. 
    // More info can be found at https://developers.fireblocks.com
    let vault_account = "0"; // Our wallet id
    let chain_id = 5; // Goerli
    let cfg = FireblocksConfig::new(
        &std::env::var("FIREBLOCKS_API_PRIVATE_KEY_PATH").expect("fireblocks secret not set"),
        &std::env::var("FIREBLOCKS_API_KEY").expect("fireblocks api key not set"),
        vault_account,
        chain_id,
    )?;
    
    let signer = FireblocksSigner::new(cfg).await;    
    let client = SignerMiddleware::new(provider, signer);
    // (this will require confirming the tx on the device)
    let tx = TransactionRequest::pay("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", 100);
    let pending_tx = client.send_transaction(tx, None).await?;

    // Get the receipt
    let _receipt = pending_tx.confirmations(3).await?;
    Ok(())
}

#[cfg(not(feature = "fireblocks"))]
fn main() {}
