// SESSION 7 — Day 1 Capstone: BTC / Sats Calculator
// Run with: cargo run -p session7_capstone
//
// Build a CLI calculator that:
//   1. Reads a BTC amount from stdin
//   2. Converts it to satoshis
//   3. Checks for dust threshold
//   4. Prints a transaction summary

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

        // TODO: parse trimmed as f64 BTC amount
        let btc: f64 = todo!();

        // TODO: convert to satoshis using BTC_TO_SATS
        let sats: u64 = todo!();

        // TODO: set to "⚠ DUST" if sats <= 546, otherwise ""
        let dust_warning: &str = todo!();

        println!("{btc} BTC = {sats} sats {dust_warning}");
    }
}
