pub const BTC_TO_SATS: u64 = 100_000_000; // TODO: set the correct value (1 BTC = ? satoshis)

/// Convert BTC to satoshis.
/// TODO: implement this function.
pub fn btc_to_sats(btc: f64) -> u64 {
    (btc * BTC_TO_SATS as f64) as u64
}

/// Return true if the amount is above the dust threshold (546 sats).
/// TODO: implement this function.
pub fn is_above_dust(sats: u64) -> bool {
    sats > 546
}
