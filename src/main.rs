use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use solana_client::rpc_client::RpcClient;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // === CONFIG ===
    let rpc_url = "https://public.rpc.solanavibestation.com";
    let slot_number: u64 = 336533075;
    let participants_file = "participants.txt";
    // ==============
    
    // Read participants from file
    let contents = fs::read_to_string(participants_file)
        .with_context(|| format!("Failed to read {}", participants_file))?;
    
    let participants: Vec<String> = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();
    
    anyhow::ensure!(!participants.is_empty(), "No participants found in file");
    
    // Connect to Solana and fetch block
    let client = RpcClient::new(rpc_url);
    let block = client
        .get_block_with_config(
            slot_number,
            solana_client::rpc_config::RpcBlockConfig {
                max_supported_transaction_version: Some(0),
                ..Default::default()
            },
        )
        .with_context(|| format!("Failed to get block at slot {}", slot_number))?;
    
    let blockhash = block.blockhash;
    println!("✅ Blockhash for slot {}: {}", slot_number, blockhash);
    
    // Use SHA256 of blockhash to pick winner
    let hash = Sha256::digest(blockhash.into_bytes());
    let num = u64::from_be_bytes(hash[..8].try_into().unwrap());
    let winner_index = (num % participants.len() as u64) as usize;
    
    println!("🏆 Winner: {}", participants[winner_index]);
    
    Ok(())
}