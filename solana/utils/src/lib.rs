pub mod account_info;

#[cfg(feature = "anchor")]
pub mod cpi;

#[cfg(feature = "anchor")]
type FeatureResult<T> = anchor_lang::Result<T>;
#[cfg(not(feature = "anchor"))]
type FeatureResult<T> = Result<T, solana_program::program_error::ProgramError>;
