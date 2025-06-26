use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::AccountMeta,
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;

const RPC_URL: &str = "http://localhost:8899";
const PROGRAM_ID: &str = "2opr1VoyXxpNePA4gcLBGPMPgrzgpyixuqDrE7EzKFWv";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup solana connection
    let rpc_url = RPC_URL.to_string();
    let client = RpcClient::new(rpc_url);

    // create and credit payer account
    let payer = Keypair::new();
    match client.request_airdrop(&payer.pubkey(), 1_000_000_000) {
        Ok(sig) => {
            println!("Airdrop requested. Signature: {}", sig);
            client.confirm_transaction_with_commitment(&sig, CommitmentConfig::finalized())?;
            println!("Airdrop confirmed. Signature: {}", sig);
        }
        Err(e) => println!("Airdrop failed: {}", e),
    }
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    // take some recent slot
    let slot_current = client.get_slot()?;
    let slots_recent = client.get_blocks(slot_current - 200, None)?;
    let slot_recent = slots_recent[0];

    // find account for Address Lookup Table
    let (alt, _) = Pubkey::find_program_address(
        &[payer.pubkey().as_ref(), &slot_recent.to_le_bytes()],
        &solana_sdk::address_lookup_table::program::id(),
    );

    // create program id
    let program_id = Pubkey::from_str(PROGRAM_ID)?;

    // create accounts vector expected by the program
    let mut accounts = vec![
        AccountMeta::new(alt, false),
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(solana_sdk::address_lookup_table::program::id(), false),
    ];
    for _ in 0..27 {
        // generate random address for ALT
        let addr = Keypair::new();
        accounts.push(AccountMeta::new_readonly(addr.pubkey(), false));
    }

    // prepare instruction data with slot id
    let instruction_data = slot_recent.to_le_bytes().to_vec();

    println!("recent_slot: {:?}", slot_recent);
    println!("alt: {:?}", alt);
    println!("payer: {:?}", payer.pubkey());

    // generate instruction to call the program
    let instruction = solana_sdk::instruction::Instruction {
        program_id,
        accounts,
        data: instruction_data,
    };

    // create and send the transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // send the transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction successful. Signature: {}", signature);

    // check compute unit consumed
    let tx_meta = client.get_transaction(&signature, UiTransactionEncoding::Json)?;
    if let Some(meta) = tx_meta.transaction.meta {
        let compute_units_consumed: Option<u64> = meta.compute_units_consumed.into();
        if let Some(compute_units_consumed) = compute_units_consumed {
            println!("CUs consumed: {}", compute_units_consumed);
        }
    }

    Ok(())
}
