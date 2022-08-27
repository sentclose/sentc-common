use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{FileId, GeneralIdFormat, PartId, SymKeyId};

/**
# Who has access to this file

When group or user, then the belongs_to must be Some(id)

The api checks if the user got access to this:
- for user -> check if belongs_to is the same as the user id
- for group -> check if the user is in this group
*/
#[derive(Serialize, Deserialize)]
pub enum BelongsToType
{
	Group,
	User,
	None,
}

#[derive(Serialize, Deserialize)]
pub struct FileData
{
	pub file_id: FileId,
	pub belongs_to: Option<GeneralIdFormat>, //can be a group or a user. if belongs to type is none then this is Option::None
	pub belongs_to_type: BelongsToType,
	pub key_id: SymKeyId,
	pub part_list: Vec<PartId>,
}
