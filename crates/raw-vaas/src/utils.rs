#[cfg(feature = "anchor")]
fn anchor_keccak(buf: &[u8]) -> [u8; 32] {
    anchor_lang::solana_program::keccak::hash(buf).0.into()
}

/// Simple keccak256 hash with configurable backend.
#[inline]
pub fn keccak256(buf: impl AsRef<[u8]>) -> [u8; 32] {
    #[cfg(all(feature = "off-chain", not(feature = "on-chain")))]
    return alloy_primitives::keccak256(buf).into();

    #[cfg(feature = "anchor")]
    #[cfg_attr(feature = "anchor", allow(unreachable_code))]
    return anchor_keccak(buf.as_ref());
}

/// Return the number of guardians to reach quorum.
#[inline]
pub fn quorum(n: usize) -> usize {
    (n * 2) / 3 + 1
}
