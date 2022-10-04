use bdk::bitcoin::{Address, Network};
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::database::MemoryDatabase;
use bdk::blockchain::EsploraBlockchain;
use bdk::wallet::AddressIndex;
use bdk::{FeeRate, Wallet, KeychainKind, SyncOptions};
use std::str::FromStr;
use std::collections::BTreeMap;
use bdk::blockchain::Blockchain;

use bdk::miniscript::psbt::PsbtExt;
use bdk::bitcoin::secp256k1::Secp256k1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut base_psbt = PartiallySignedTransaction::from_str("TODO: insert the psbt created in step 3 here")?;
    let signed_psbts = vec![
         // TODO: Paste each participant's PSBT here
         "insertyourpsbthere",
         "insertanotherpsbthere",
    ];

    for mut psbt in signed_psbts {
        let psbt = PartiallySignedTransaction::from_str(psbt)?;
        base_psbt.combine(psbt)?;
    }

    let secp = Secp256k1::new();
    let psbt = base_psbt.finalize(&secp).unwrap();
    let finalized_tx = psbt.extract_tx();
    dbg!(finalized_tx.txid());

    let blockchain = EsploraBlockchain::new("https://blockstream.info/testnet/api", 20);
    dbg!(blockchain.broadcast(&finalized_tx));
    Ok(())
}
