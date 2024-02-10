/// Return the number of guardians to reach quorum.
#[inline]
pub fn quorum(n: usize) -> usize {
    (n * 2) / 3 + 1
}
