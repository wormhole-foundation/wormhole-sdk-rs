use alloy_primitives::FixedBytes;

#[cfg(feature = "anchor")]
fn anchor_keccak(buf: &[u8]) -> FixedBytes<32> {
    anchor_lang::solana_program::keccak::hash(buf)
}

/// Simple keccak256 hash with configurable backend.
#[inline]
pub fn keccak256(buf: impl AsRef<[u8]>) -> FixedBytes<32> {
    #[cfg(not(feature = "anchor"))]
    return alloy_primitives::keccak256(buf);

    #[cfg(feature = "anchor")]
    anchor_keccak(buf.as_ref())
}

/// Return the number of guardians to reach quorum.
#[inline]
pub fn quorum(n: usize) -> usize {
    (n * 2) / 3 + 1
}
