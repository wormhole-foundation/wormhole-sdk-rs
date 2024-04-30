pub mod zero_copy;

use wormhole_raw_vaas::Vaa;

/// Representation of VAA versions.
#[non_exhaustive]
pub enum VaaVersion<'a> {
    V1(Vaa<'a>),
}
