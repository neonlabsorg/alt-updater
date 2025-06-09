use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Set up the RPC client (using devnet for this example)
    let rpc_url = "http://127.0.0.1:8899".to_string();
    let client = RpcClient::new(rpc_url);

    // 2. Load the payer keypair (in a real app, use proper key management)
    let payer = Keypair::new(); // For demo only - generates a new keypair
    println!("Payer public key: {}", payer.pubkey());

    // 3. Airdrop some SOL to the payer (only works on devnet/testnet)
    match client.request_airdrop(&payer.pubkey(), 1_000_000_000) {
        Ok(sig) => {
            println!("Airdrop requested. Signature: {}", sig);
            // Wait for confirmation
            client.confirm_transaction(&sig)?;
        }
        Err(e) => println!("Airdrop failed: {}", e),
    }
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    
    // 4. Specify the program ID you want to call
    let program_id_str = "AXb5znZEUMTe3U6qoj168sTtiueQNNnP5YThMi9Mntuj";
    let program_id = Pubkey::from_str(program_id_str)?;
    println!("Calling program with ID: {}", program_id);

    // 5. Create an instruction to call the program
    // Note: You'll need to know the exact instruction format expected by the program
    // This is a placeholder - replace with actual instruction data for your program
    let instruction_data = vec![0; 10]; // Example data

    let instruction = solana_sdk::instruction::Instruction {
        program_id,
        accounts: vec![], // Add required accounts here
        data: instruction_data,
    };

    // 6. Create and send the transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // 7. Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction successful. Signature: {}", signature);

    Ok(())
}
