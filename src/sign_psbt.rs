use bdk::wallet::signer::{SignerWrapper, SignerContext, SignerOrdering, SignOptions};
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::{KeychainKind, Wallet};
use bdk::bitcoin::Network;
use bdk::bitcoin::PrivateKey;
use bdk::database::MemoryDatabase;
use bdk::wallet::AddressIndex;
use std::fs::File;
use std::io::Read;
use bdk::bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
use std::str::FromStr;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Create a BDK wallet
    let mut wallet = Wallet::new(
        // TODO: insert your descriptor here
        "tr(youshouldputyourdescriptorhere)",
         None,
         Network::Testnet,
         MemoryDatabase::new()
     )?;

    // Step 1.5: Print the first address (so that we can check we all have the same wallet)
    println!("{:?}", wallet.get_address(AddressIndex::New)?);

    // // Step 2: Add the BDK signer
    let mut private_key_str = String::new();
    File::open("key.txt")?.read_to_string(&mut private_key_str)?;
    println!("{}", private_key_str);
    let private_key = PrivateKey::from_str(&private_key_str)?;
    let signer = SignerWrapper::new(private_key, SignerContext::Tap { is_internal_key: false });

    wallet.add_signer(
        KeychainKind::External,
        SignerOrdering(0),
        Arc::new(signer)
    );

    // // Step 3: Sign the transaction
    let mut psbt = PartiallySignedTransaction::from_str("TODO: paste the PSBT obtained in step 3 here")?;
    let finalized = wallet.sign(&mut psbt, SignOptions::default());
    println!("{}", psbt);

    Ok(())
}
