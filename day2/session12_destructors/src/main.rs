// SESSION 12 — Day 2 Capstone: UTXO Wallet
// Run with: cargo run -p session12_destructors
//
// Build a mini in-memory UTXO wallet that:
//   1. Holds a list of UTXOs (txid, vout, value)
//   2. Can add and spend UTXOs
//   3. Tracks a wallet lock released automatically on drop
//   4. Prints available balance

#[derive(Debug)]
struct Utxo {
    txid: String,
    vout: u32,
    value_sats: u64,
}

struct WalletLock {
    path: String,
}

// TODO: implement Drop for WalletLock — print "Released lock: <path>" when dropped.
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

    // TODO: return total balance across all UTXOs.
    fn balance(&self) -> u64 {
        todo!()
    }

    // TODO: remove the UTXO at `index` and return its value. Panic if out of bounds.
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
