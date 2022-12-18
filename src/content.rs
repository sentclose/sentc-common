use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{CategoryId, ContentId, GroupId, UserId};

#[derive(Serialize, Deserialize)]
pub struct CreateData
{
	pub cat_ids: Vec<CategoryId>,
	pub item: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContentCreateOutput
{
	pub content_id: CategoryId,
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
	pub access_from_group: Option<GroupId>,
}
