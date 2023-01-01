use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::{CategoryId, ContentId, GroupId, UserId};

#[derive(Serialize, Deserialize)]
pub struct CreateData
{
	#[serde(skip_serializing_if = "Option::is_none")]
	pub category: Option<CategoryId>,
	pub item: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContentCreateOutput
{
	pub content_id: ContentId,
}

#[derive(Serialize, Deserialize)]
pub struct ListContentItem
{
	pub id: ContentId,
	pub item: String,
	pub belongs_to_group: Option<GroupId>,
	pub belongs_to_user: Option<UserId>,
	pub creator: UserId,
	pub time: u128,
	pub category: Option<CategoryId>,
	pub access_from_group: Option<GroupId>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentItemAccess
{
	pub access: bool,
	pub access_from_group: Option<GroupId>,
}
