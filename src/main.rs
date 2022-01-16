use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::signature::read_keypair_file;
use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Transaction;
use std::time::{Instant};
use rand::prelude::*;


fn main() {
    let _cluster_url_local =  "http://localhost:8899".to_string();
    let _cluster_url_devnet =  "https://api.devnet.solana.com".to_string();
    let _cluster_url_devnet_gg =  "https://psytrbhymqlkfrhudd.dev.genesysgo.net:8899".to_string();

    let from_wallet_key = format!("keys/{}.json", get_random_key());

    // Initialize wallet to send transactions from
    let from_wallet = read_keypair_file(&*shellexpand::tilde(&from_wallet_key))
        .expect("Example requires a keypair file");
    println!("Source account address {}", from_wallet.pubkey());

    // Create RPC client for communicating with the Solana cluster
    let rpc = RpcClient::new_with_commitment(_cluster_url_devnet_gg, CommitmentConfig::processed());

    #[allow(while_true)]
    while true {
        // Create counter
        let mut count = 0;
        // Get random wallet to send transfers to
        let to_wallet = create_to_keypair();
        // Check not to send itself
        if from_wallet == to_wallet {
            println!("Addresses are the same! Skipping.");
            continue;
        }
        // Initialize timer
        let start = Instant::now();
        // Execute in batches
        while count <= 10 {
            if let Err(err) = run(&from_wallet, &to_wallet, &rpc) {
                eprintln!("{:?}", err);
                std::process::exit(1);
            }
            println!("Transaction successful.");
            count += 1;
        }
        // Get total duration of all transactions in batch
        let duration = start.elapsed();

        // Add a zero after 1000 for every zero in the `count` to get the TPS
        let duration_ms = duration.as_millis() as f64;
        let tps: f64 = 10000.0/duration_ms;
        println!("Average transaction took {:?}ms.", duration.as_millis()/10);
        println!("TPS is {:.2}", tps);
        
        // std::thread::sleep(Duration::from_millis(100));
    }
}

fn run(from_wallet: &Keypair, to_wallet: &Keypair, rpc: &RpcClient) -> Result<Signature, ClientError> {
    // Build create account instruction
    let instruction = solana_sdk::system_instruction::transfer(
        &from_wallet.pubkey(),
        &to_wallet.pubkey(),
        500
    );

    // Build transaction
    let signers = [&*from_wallet];
    let instructions = vec![instruction];

    let recent_hash: solana_sdk::hash::Hash = rpc.get_latest_blockhash()?;

    let txn = Transaction::new_signed_with_payer(
        &instructions,
        Some(&from_wallet.pubkey()),
        &signers,
        recent_hash,
    );

    rpc.send_transaction_with_config(
        &txn,
        RpcSendTransactionConfig {
            skip_preflight: true,
            ..RpcSendTransactionConfig::default()
        }
    )
}

// Generates a random number between 0 - 9
fn get_random_key() -> i32 {
    let mut rng = thread_rng();
    let wallet_key = rng.gen_range(0..10);
    wallet_key
}

fn create_to_keypair() -> Keypair {
    let to_wallet_key = format!("keys/{}.json", get_random_key());
    // Create a new Keypair from file
    let to_wallet = read_keypair_file(&*shellexpand::tilde(&to_wallet_key))
        .expect("Example reqiures a keypair file");
    println!("Target account address: {}", to_wallet.pubkey());
    to_wallet
}
