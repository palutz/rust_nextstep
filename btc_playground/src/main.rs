use bdk::blockchain::ElectrumBlockchain;
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bdk::{bitcoin, SyncOptions, Wallet};

fn calculate_size<T>() -> usize {
    let size = core::mem::size_of::<T>();
    println!("a has type T and size = {size}");
    size
}

fn main() -> Result<(), bdk::Error> {
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    let wallet = Wallet::new(
        "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
        Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
        bitcoin::Network::Testnet,
        MemoryDatabase::default(),
    )?;

    wallet.sync(&blockchain, SyncOptions::default())?;

    println!("Descriptor balance: {} SAT", wallet.get_balance()?);

    Ok(())
}

#[cfg(test)]
#[test]
fn test_calcsize() {
    println!("i32 size is {:?}", calculate_size::<i32>());
    println!("i64 size is {:?}", calculate_size::<i64>());
    println!("i64 size is {:?}", calculate_size::<u64>());
    println!("u8 size is {:?}", calculate_size::<u8>());
    println!("u8 size is {:?}", calculate_size::<u16>());
    assert!(true);
}
#[test]
fn calc_size() {
    let s = core::mem::size_of::<u64>();
    println!("{s}");
    assert!(true);
}