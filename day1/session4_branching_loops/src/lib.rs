/// Classify a fee rate (sats/vbyte) into a tier.
/// < 2  → "low"
/// < 10 → "medium"
/// >= 10 → "high"
/// TODO: implement this function.
pub fn fee_tier(sats_per_vbyte: u64) -> &'static str {
    todo!()
}

/// Sum all values in a UTXO set (slice of satoshi amounts).
/// TODO: implement this function.
pub fn total_value(utxos: &[u64]) -> u64 {
    todo!()
}
