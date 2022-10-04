use bdk::bitcoin::{Address, Network};
use bdk::database::MemoryDatabase;
use bdk::blockchain::EsploraBlockchain;
use bdk::wallet::AddressIndex;
use bdk::{FeeRate, Wallet, KeychainKind, SyncOptions};
use std::str::FromStr;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Create a BDK wallet
    let wallet = Wallet::new(
        // TODO: insert your descriptor here
        "tr(youshouldputyourdescriptorhere)",
        None,
        Network::Testnet,
        MemoryDatabase::new()
    )?;

    // Step 2: Print the first address
    println!("Deposit funds here: {:?}", wallet.get_address(AddressIndex::New)?);

    // Step 3: Deposit funds
    // Use some testnet faucet, such as https://bitcoinfaucet.uo1.net/send.php

    // Step 4: Print balance
    let blockchain = EsploraBlockchain::new("https://blockstream.info/testnet/api", 20);
    wallet.sync(&blockchain, SyncOptions::default())?;
    println!("{:#?}", wallet.get_balance()?);

    // Step 5: Create a transaction
    let faucet_address = Address::from_str("tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt")?;

    // We need to specify with which policy funds will be spent. Our current wallet contains 3
    // policies: the key path spend, and two leaves in the script path spend.
    let wallet_policy = wallet.policies(KeychainKind::External)?.unwrap();
    let mut path = BTreeMap::new();
    // We need to use the first leaf of the script path spend, hence the second policy
    // If you're not sure what's happening here, no worries, this is bit tricky :)
    // You can learn more here: https://docs.rs/bdk/latest/bdk/wallet/tx_builder/struct.TxBuilder.html#method.policy_path
    path.insert(wallet_policy.id, vec![1]);

    let mut tx_builder = wallet.build_tx();
    tx_builder
         .drain_wallet()
         .drain_to(faucet_address.script_pubkey())
         .fee_rate(FeeRate::from_sat_per_vb(3.0))
         .policy_path(path, KeychainKind::External);

    let (psbt, _details) = tx_builder.finish()?;

    println!("{}", psbt);
    Ok(())
}
