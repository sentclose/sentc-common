use alloc::string::String;

use serde::{Deserialize, Serialize};

use crate::{CategoryId, ContentId, GroupId, UserId};

#[derive(Serialize, Deserialize)]
pub struct CreateData
{
	pub cat_id: Option<CategoryId>,
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
	pub cat_id: Option<CategoryId>,
	pub access_from_group: Option<GroupId>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentItemAccess
{
	pub access: bool,
	pub access_from_group: Option<GroupId>,
}

#[derive(Serialize, Deserialize)]
pub struct ContentCategoryCreateInput
{
	pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContentCategoryOutput
{
	pub cat_id: CategoryId,
}

#[derive(Serialize, Deserialize)]
pub struct ListContentCategoryItem
{
	pub cat_id: CategoryId,
	pub name: String,
	pub time: u128,
}
