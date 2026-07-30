/// Classify a fee rate (sats/vbyte) into a tier.
/// < 2  → "low"
/// < 10 → "medium"
/// >= 10 → "high"
/// TODO: implement this function.
pub fn fee_tier(sats_per_vbyte: u64) -> &'static str {
    todo!()
}

/// Return true if a transaction is confirmed (confirmations >= 6).
/// TODO: implement this function.
pub fn is_confirmed(confirmations: u32) -> bool {
    todo!()
}
