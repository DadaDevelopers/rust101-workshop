/// Classify a fee rate (sats/vbyte) into a tier.
/// < 2  → "low"
/// < 10 → "medium"
/// >= 10 → "high"
/// TODO: implement this function.
pub fn fee_tier(sats_per_vbyte: u64) -> &'static str {
    if sats_per_vbyte < 2 {
        "low"
    } else if sats_per_vbyte < 10 {
        "medium"
    } else {
        "high"
    }
}

/// Return true if a transaction is confirmed (confirmations >= 6).
/// TODO: implement this function.
pub fn is_confirmed(confirmations: u32) -> bool {
    confirmations >= 6
    
}
