use alloc::string::String;

use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
pub struct ServerOutput<T: Serialize>
{
	pub status: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub err_msg: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub err_code: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result: Option<T>,
}

impl<'de, T: Serialize + Deserialize<'de>> ServerOutput<T>
{
	pub fn from_string(v: &'de str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}
