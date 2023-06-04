use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::crypto::SignHead;
use crate::{FileId, FileSessionId, GeneralIdFormat, PartId, UserId};

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
pub struct FilePartListItem
{
	pub part_id: PartId,
	pub sequence: i32,
	pub extern_storage: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FileData
{
	pub file_id: FileId,
	pub master_key_id: String,
	pub owner: UserId,
	pub belongs_to: Option<GeneralIdFormat>, //can be a group or a user. if belongs to type is none then this is Option::None
	pub belongs_to_type: BelongsToType,
	pub encrypted_key: String,
	pub encrypted_key_alg: String,
	pub encrypted_file_name: Option<String>,
	pub part_list: Vec<FilePartListItem>,
}

//__________________________________________________________________________________________________

#[derive(Serialize, Deserialize)]
pub struct FileRegisterInput
{
	pub encrypted_key: String,
	pub encrypted_key_alg: String,
	pub master_key_id: String, //can be group key or user private / public key pair id
	pub belongs_to_id: Option<GeneralIdFormat>,
	pub belongs_to_type: BelongsToType,
	pub encrypted_file_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FileRegisterOutput
{
	pub file_id: FileId,
	pub session_id: FileSessionId,
}

#[derive(Serialize, Deserialize)]
pub struct FileNameUpdate
{
	pub encrypted_file_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FilePartRegisterOutput
{
	pub part_id: PartId,
}

//__________________________________________________________________________________________________

#[derive(Serialize, Deserialize)]
pub struct FileHead
{
	pub key: String,
	pub sign: Option<SignHead>,
	pub sym_key_alg: String,
}
