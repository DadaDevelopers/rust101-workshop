// SESSION 12 — Day 2 Capstone: UTXO Wallet (group project)
// Run with: cargo run
//
// Groups build a mini in-memory UTXO wallet that:
//   1. Holds a list of UTXOs (txid, vout, value)
//   2. Can add and spend UTXOs
//   3. Tracks a wallet lock file — dropped automatically when the wallet closes
//   4. Prints a summary of available balance
//
// Work through the TODOs below, then present your running wallet to the room.

#[derive(Debug)]
struct Utxo {
    txid: String,
    vout: u32,
    value_sats: u64,
}

struct WalletLock {
    path: String,
}

// TODO: implement Drop for WalletLock so it prints "Released lock: <path>" when dropped.
impl Drop for WalletLock {
    fn drop(&mut self) {
        todo!()
    }
}

struct Wallet {
    utxos: Vec<Utxo>,
    _lock: WalletLock,
}

impl Wallet {
    // TODO: create a new Wallet, acquiring a lock on "wallet.lock".
    fn new() -> Self {
        todo!()
    }

    // TODO: add a UTXO to the wallet.
    fn deposit(&mut self, txid: &str, vout: u32, value_sats: u64) {
        todo!()
    }

    // TODO: return the total balance across all UTXOs.
    fn balance(&self) -> u64 {
        todo!()
    }

    // TODO: spend the UTXO at `index`, removing it from the list and returning its value.
    // Panic if the index is out of bounds.
    fn spend(&mut self, index: usize) -> u64 {
        todo!()
    }
}

fn main() {
    let mut wallet = Wallet::new();

    wallet.deposit("abc123", 0, 50_000);
    wallet.deposit("def456", 1, 150_000);
    wallet.deposit("ghi789", 0, 300_000);

    println!("Balance: {} sats", wallet.balance());

    let spent = wallet.spend(1);
    println!("Spent: {spent} sats");
    println!("Remaining balance: {} sats", wallet.balance());
}
