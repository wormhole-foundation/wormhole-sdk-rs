mod message;
pub use message::Message;

pub mod gov;
pub mod portal;

/// Trait to capture common payload behavior.
pub trait Payload: crate::Readable + crate::Writeable + Clone + std::fmt::Debug {}
