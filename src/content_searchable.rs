use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::SymKeyId;

#[derive(Serialize, Deserialize)]
pub struct SearchableCreateOutput
{
	pub hashes: Vec<String>,
	pub alg: String,
	pub key_id: SymKeyId,
}
