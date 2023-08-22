use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::CoreDeployment;

pub static MAINNET: Lazy<HashMap<u16, CoreDeployment>> = Lazy::new(|| todo!());
pub static TESTNET: Lazy<HashMap<u16, CoreDeployment>> = Lazy::new(|| todo!());

// TODO: deploy info
