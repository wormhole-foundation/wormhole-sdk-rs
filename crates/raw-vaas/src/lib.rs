mod protocol;
pub use protocol::{GuardianSetSig, Header, Payload, Vaa};

mod payloads;
pub use payloads::{cctp, core, token_bridge, GovernanceHeader, GovernanceMessage};

pub mod utils;

pub mod support;

#[cfg(all(feature = "off-chain", feature = "on-chain"))]
compile_error!(
    "Only one of `off-chain` or `on-chain` can be enabled. N.b. `off-chain` is on by default."
);
