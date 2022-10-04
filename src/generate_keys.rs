use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
use bdk::bitcoin::{PrivateKey, PublicKey, Network};
use bdk::keys::GeneratableDefaultOptions;
use bdk::keys::GeneratedKey;
use bdk::miniscript;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 0: abort if we already created a key
    let path = Path::new("key.txt");
    if path.exists() {
        panic!("Key already created");
    }

    // Step 1: generate a WIF
    let private_key: GeneratedKey<_, miniscript::Tap> = PrivateKey::generate_default()?;
    let mut private_key = private_key.into_key();
    private_key.network = Network::Testnet;

    // Step 2: print the xpub to stdout
    let secp = Secp256k1::new();
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    println!("Public key: {}", public_key);

    // Step 3: save the WIF in a file
    let mut file = File::create("key.txt")?;
    file.write_all(private_key.to_string().as_bytes())?;

    Ok(())
}
