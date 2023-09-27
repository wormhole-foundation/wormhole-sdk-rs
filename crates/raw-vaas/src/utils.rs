/// Simple keccak256 hash with configurable backend.
#[cfg(all(feature = "off-chain", not(feature = "on-chain")))]
#[inline]
pub fn keccak256(buf: impl AsRef<[u8]>) -> [u8; 32] {
    alloy_primitives::keccak256(buf).into()
}

/// Return the number of guardians to reach quorum.
#[inline]
pub fn quorum(n: usize) -> usize {
    (n * 2) / 3 + 1
}
