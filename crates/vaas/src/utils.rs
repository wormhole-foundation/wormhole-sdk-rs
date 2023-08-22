use alloy_primitives::FixedBytes;

/// Simple keccak256 hash with configurable backend.
#[inline]
pub fn keccak256(buf: impl AsRef<[u8]>) -> FixedBytes<32> {
    alloy_primitives::keccak256(buf)
}

/// Return the number of guardians to reach quorum.
#[inline]
pub fn quorum(n: usize) -> usize {
    (n * 2) / 3 + 1
}
