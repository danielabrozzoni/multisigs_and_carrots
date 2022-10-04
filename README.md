# Multisig & carrots

This repository contains all the code to build a taproot multisig with your friends, and to spend from it. It's been used first to do the BDK workshop in the [Azores Unconference](https://btcazores.com/), where we created all together this transaction, spending from a taproot multisig: https://blockstream.info/testnet/tx/d40c55286933dfc2debb6b34832070401db33f53c78143bfb48717b79d237bdf?expand. If you want a transcript of the expertiment: https://twitter.com/michaelfolkson/status/1573644913224425473?s=20&t=NXMkPwrfZRvrUdPb5pBA2g

You can use the code to re-create this workshop at some conference. You can use it to create a Taproot night with your friends. You can use it by yourself to learn how BDK works. You can use it however you want. :)

# The details

The expertiment is composed of various participants, which will hold private keys and sign the transaction, and one coordinator, whose job is to, well, coordinate: creating the descriptor, creating the transaction, merging the signatures together and broadcasting the transaction.

At each step, I suggest commenting the whole file, and then uncommenting it, line by line, while you read it. If some parts are not clear, you are encouraged to add `dbg!()` and `println!()` statements, and run the code to see what happens. Don't be scared of experimenting :)

Some data needs to be exchanged between the coordinator and the participants (public keys, descriptor, PSBTs). In the real expertiment we chose to use telegram, as it works well on desktop, but there are some obvious privacy concerns. Find a messagging app that works for you, or, if you're a real hacker, build a web page for submitting the data.

This article will give you a brief introduction to Taproot descriptors: https://bitcoinops.org/en/preparing-for-taproot/#taproot-descriptors

In this example, we put a dummy key in the key spend path (spending with just a key is too easy for us :grin:), and two different policies in the script spend path:

1. You can spend immediately if at least half of the participants sign
2. You can spend after 100 with a recovery key - just in case too many participants lose their keys :)

The taptree ends up looking like this:

# The steps

The expertiment is composed of 5 steps:

1. Participants generating private keys, and sharing the public ones
2. Coordinator generating the descriptor
3. Coordinator sending money to the descriptor, then creating a tx for spending
4. Participants signing the PSBTs
5. Coordinator merges the signatures together and broadcasts

## Step 1: generating the keys (everyone)

This will create a WIF key, print the public one to stdout, and write the private one to `key.txt`. Make sure not to delete `key.txt`, or you'll have to use the recovery key!

Run with:

```
cargo run --bin generate_keys
```

## Step 2: generating the descriptor (coordinator only)

Only the coordinator needs to run this step. After gathering all the participants' public keys, insert them in the "keys" vector. The descriptor will be printed to stdout. Send it to the group, as you'll need it for the next steps.

Run with:
```
cargo run --bin generate_descriptor
```

## Step 3: creating a transaction spending the funds (coordinator only)

Only the coordinator needs to run this step. You should NOT run this file all at once - instead, uncomment Step 1 and 2, insert the descriptor obtained in step 2 where indicated, run the file and send some testnet coins to the address displayed. Only then uncomment the rest.
The code will print a PSBT - send it to the group, you'll need it for the next step

Run with:
```
cargo run --bin create_psbt
```

## Step 4: signing the transaction

In this step everyone will sign the PSBT obtained in step 3. Make sure that you still have the `key.txt` file, remove the TODOs and insert your descriptor and your PSBT instead. The file will print to stdout the signed PSBT - send it to the coordinator.

Run with:

```
cargo run --bin sign_transaction
```

## Step 5: merging the signatures together, broadcasting (coordinator only)

Last step! The coordinator will gather everyone's transactions, insert them in the file, which will combine them all together and try to finalize and extract a bitcoin transaction. If all goes well, the transaction will be extracted and broadcasted :tada:

```
cargo run --bin combine_psbts
```

# Going further

If you're up for a challenge, try to:

1. Use BIP32 keys instead of WIF keys
2. Spend using the recovery path (you need to wait 100 blocks first!)
3. Sync and broadcast the transaction using a different backend, such as Electrum or Bitcoin Core (we use esplora both in `create_psbt` and `combine_psbts`)
4. Spend using the key spend path (hint: you need to change the descriptor a bit, or find a way to break secp256k1)
