mod attestation;
pub use attestation::Attestation;

mod transfer;
pub use transfer::Transfer;

mod transfer_with_message;
pub use transfer_with_message::TransferWithMessage;

use crate::payloads::Payload;

impl Payload for Transfer {}
impl Payload for Attestation {}
impl Payload for TransferWithMessage {}
