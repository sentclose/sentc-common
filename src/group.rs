use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::{EncryptionKeyPairId, GroupId, SignKeyPairId, SymKeyId, UserId};

#[derive(Serialize, Deserialize)]
pub struct CreateData
{
	pub encrypted_group_key: String,
	pub group_key_alg: String,
	pub encrypted_group_key_alg: String,
	pub encrypted_private_group_key: String,
	pub public_group_key: String,
	pub keypair_encrypt_alg: String,
	pub creator_public_key_id: EncryptionKeyPairId,

	//only for user group key rotation not for normal
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encrypted_sign_key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub verify_key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub keypair_sign_alg: Option<String>,
}

impl CreateData
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub struct GroupCreateOutput
{
	pub group_id: GroupId,
}

#[derive(Serialize, Deserialize)]
pub struct KeyRotationData
{
	pub encrypted_group_key_by_user: String, //encrypted by invoker public key
	pub group_key_alg: String,
	pub encrypted_group_key_alg: String, //info about how the encrypted group key was encrypted by the pk from the invoker (important for the server)
	pub encrypted_private_group_key: String,
	pub public_group_key: String,
	pub keypair_encrypt_alg: String,
	pub encrypted_group_key_by_ephemeral: String,
	pub ephemeral_alg: String,
	pub encrypted_ephemeral_key: String, //encrypted by the old group key. encrypt this key with every other member public key on the server
	pub previous_group_key_id: SymKeyId,
	pub invoker_public_key_id: EncryptionKeyPairId,

	//only for user group key rotation not for normal
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encrypted_sign_key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub verify_key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub keypair_sign_alg: Option<String>,
}

impl KeyRotationData
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub struct KeyRotationStartServerOutput
{
	pub group_id: GroupId,
	pub key_id: SymKeyId,
}

#[derive(Serialize, Deserialize)]
pub struct KeyRotationInput
{
	pub encrypted_ephemeral_key_by_group_key_and_public_key: String,
	pub encrypted_group_key_by_ephemeral: String,
	pub ephemeral_alg: String,
	pub encrypted_eph_key_key_id: EncryptionKeyPairId, //the public key id which was used to encrypt the eph key on the server.
	pub previous_group_key_id: SymKeyId,               //use this in the client sdk to load the right group key from the storage
	pub time: u128,
	pub new_group_key_id: SymKeyId, //to done the key rotation on the server
}

impl KeyRotationInput
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub struct DoneKeyRotationData
{
	pub encrypted_new_group_key: String,
	pub public_key_id: EncryptionKeyPairId,
	pub encrypted_alg: String, //the alg of the public key
}

impl DoneKeyRotationData
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

/**
# the current keys of a group

contains:
- encrypted group key
- encrypted private group (e.g. for sub group)
- public key
- and which public key was used to encrypt the group key

A group can have multiple of these structs for each key rotation
*/
#[derive(Serialize, Deserialize)]
pub struct GroupKeyServerOutput
{
	pub encrypted_group_key: String,
	pub group_key_alg: String,
	pub group_key_id: SymKeyId,
	pub encrypted_private_group_key: String,
	pub public_group_key: String,
	pub keypair_encrypt_alg: String,
	pub key_pair_id: EncryptionKeyPairId,
	pub user_public_key_id: EncryptionKeyPairId, //to know what private key we should use to decrypt
	pub time: u128,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub encrypted_sign_key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub verify_key: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub keypair_sign_alg: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub keypair_sign_id: Option<SignKeyPairId>,
}

impl GroupKeyServerOutput
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub enum GroupUserAccessBy
{
	User,
	Parent(GroupId),
	GroupAsUser(GroupId),
	GroupAsUserAsParent
	{
		parent: GroupId,
		group_as_user: GroupId,
	},
}

/**
# The data about the group from the server

save this in the sdk impl storage
*/
#[derive(Serialize, Deserialize)]
pub struct GroupServerData
{
	pub group_id: GroupId,
	pub parent_group_id: Option<GroupId>,
	pub keys: Vec<GroupKeyServerOutput>,
	pub key_update: bool,
	pub rank: i32,
	pub created_time: u128,
	pub joined_time: u128,
	pub access_by: GroupUserAccessBy,
}

impl GroupServerData
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

/**
This values can be changed while the user still caches the old values in the client
*/
#[derive(Serialize, Deserialize)]
pub struct GroupDataCheckUpdateServerOutput
{
	pub key_update: bool,
	pub rank: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GroupUserListItem
{
	pub user_id: UserId,
	pub rank: i32,
	pub joined_time: u128,
	pub user_type: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GroupKeysForNewMember
{
	pub encrypted_group_key: String, //base64 encoded
	pub alg: String,                 //the group key alg
	pub encrypted_alg: String,       //the alg of the public encryption
	pub key_id: SymKeyId,
	pub user_public_key_id: EncryptionKeyPairId,
}

#[derive(Serialize, Deserialize)]
pub struct GroupKeysForNewMemberServerInput
{
	pub keys: Vec<GroupKeysForNewMember>,
	pub key_session: bool, //when there are mor than 100 keys in this group -> the client wants a session
}

impl GroupKeysForNewMemberServerInput
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub struct GroupJoinReqList
{
	pub user_id: UserId,
	pub time: u128,
	pub user_type: i32, // 0 = normal user, 1 = group as member
}

#[derive(Serialize, Deserialize)]
pub struct GroupInviteReqList
{
	pub group_id: GroupId,
	pub time: u128,
}

#[derive(Serialize, Deserialize)]
pub struct GroupInviteServerOutput
{
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String>,
	pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupAcceptJoinReqServerOutput
{
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String>,
	pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupChangeRankServerInput
{
	pub changed_user_id: UserId,
	pub new_rank: i32,
}

impl GroupChangeRankServerInput
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub struct ListGroups
{
	pub group_id: GroupId,
	pub time: u128,
	pub joined_time: u128,
	pub rank: i32,
	pub parent: Option<GroupId>,
}
