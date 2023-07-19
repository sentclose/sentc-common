use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::SymKeyId;

#[derive(Serialize, Deserialize)]
pub struct SortableEncryptOutput
{
	pub number: u64,
	pub alg: String,
	pub key_id: SymKeyId,
}
