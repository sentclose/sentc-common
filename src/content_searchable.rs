use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{CategoryId, ContentId, SymKeyId};

#[derive(Serialize, Deserialize)]
pub struct ListSearchItem
{
	pub id: ContentId,
	pub item_ref: String,
	pub time: u128,
}

#[derive(Serialize, Deserialize)]
pub struct SearchCreateData
{
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category: Option<CategoryId>,
	pub item_ref: String,
	pub hashes: Vec<String>,
	pub alg: String,
	pub key_id: SymKeyId,
}
