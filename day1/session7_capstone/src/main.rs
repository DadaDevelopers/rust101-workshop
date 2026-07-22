// SESSION 7 — Day 1 Capstone: BTC / Sats Calculator
// Run with: cargo run
//
// Groups build a CLI calculator that:
//   1. Reads a BTC amount from the user
//   2. Converts it to satoshis
//   3. Classifies the fee tier given a fee rate
//   4. Checks for dust and overflow
//   5. Prints a transaction summary
//
// Work through the TODOs below, then present your output to the room.

use std::io::{self, BufRead};

const BTC_TO_SATS: u64 = 100_000_000;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.expect("failed to read line");
        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        // TODO: parse trimmed as f64 BTC amount (hint: str::parse)
        let btc: f64 = todo!();

        // TODO: convert to satoshis using BTC_TO_SATS
        let sats: u64 = todo!();

        // TODO: check for dust threshold (546 sats)
        let dust_warning: &str = todo!(); // "DUST" or ""

        println!("{btc} BTC = {sats} sats {dust_warning}");
    }
}
