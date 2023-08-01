use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::group::{CreateData, GroupHmacData, GroupInviteReqList, GroupKeyServerOutput, GroupKeysForNewMemberServerInput, GroupNewMemberLightInput};
use crate::{DeviceId, EncryptionKeyPairId, GroupId, SignKeyPairId, SymKeyId, UserId};

#[derive(Serialize, Deserialize)]
pub struct CaptchaCreateOutput
{
	pub captcha_id: String,
	pub png: String, //base64 encoded
}

#[derive(Serialize, Deserialize)]
pub struct CaptchaInput
{
	pub captcha_solution: String,
	pub captcha_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MasterKey
{
	pub master_key_alg: String,
	pub encrypted_master_key: String, //base64 encoded master key
	pub encrypted_master_key_alg: String,
}

#[derive(Serialize, Deserialize)]
pub struct KeyDerivedData
{
	pub derived_alg: String,
	pub client_random_value: String, //don't use the enum for out, we will get the enum form the derived alg on the server (because the rand value is only used on the server)
	pub hashed_authentication_key: String,

	//pub/pri encrypt decrypt
	pub public_key: String,
	pub encrypted_private_key: String,
	pub keypair_encrypt_alg: String,

	//sign/verify
	pub verify_key: String,
	pub encrypted_sign_key: String,
	pub keypair_sign_alg: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserIdentifierAvailableServerInput
{
	pub user_identifier: String,
}

impl UserIdentifierAvailableServerInput
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
pub struct UserIdentifierAvailableServerOutput
{
	pub user_identifier: String,
	pub available: bool,
}

impl UserIdentifierAvailableServerOutput
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
pub struct UserDeviceRegisterInput
{
	pub master_key: MasterKey,
	pub derived: KeyDerivedData,
	pub device_identifier: String, //with this the user is called for login. can be username or an id, or an email
}

#[derive(Serialize, Deserialize)]
pub struct UserDeviceRegisterOutput
{
	pub device_id: DeviceId, //device id is also the public key id (keypair_encrypt_id)
	pub token: String,
	pub device_identifier: String,
	pub public_key_string: String,
	pub keypair_encrypt_alg: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDeviceDoneRegisterInputLight
{
	pub user_group: GroupNewMemberLightInput,
	pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDeviceDoneRegisterInput
{
	pub user_keys: GroupKeysForNewMemberServerInput,
	pub token: String,
}

/**
# Register Data for the server api

send this after register to the server
*/
#[derive(Serialize, Deserialize)]
pub struct RegisterData
{
	pub device: UserDeviceRegisterInput, //the first device of the user
	pub group: CreateData,               //the user group data
}

impl RegisterData
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
pub struct RegisterServerOutput
{
	pub user_id: UserId,
	pub device_id: DeviceId,
	pub device_identifier: String,
}

impl RegisterServerOutput
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
pub struct ChangePasswordData
{
	pub new_derived_alg: String,
	pub new_client_random_value: String,
	pub new_hashed_authentication_key: String,
	pub new_encrypted_master_key: String,
	pub new_encrypted_master_key_alg: String,
	pub old_auth_key: String,
}

impl ChangePasswordData
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
pub struct ResetPasswordData
{
	pub client_random_value: String, //don't use the enum for out, we will get the enum form the derived alg on the server (because the rand value is only used on the server)
	pub hashed_authentication_key: String,
	pub master_key: MasterKey,
	pub derived_alg: String,
	pub encrypted_private_key: String,
	pub encrypted_sign_key: String,
}

impl ResetPasswordData
{
	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}

	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}

#[derive(Serialize, Deserialize)]
pub struct UserPublicKeyData
{
	pub public_key_pem: String,
	pub public_key_alg: String,
	pub public_key_id: EncryptionKeyPairId,
	pub public_key_sig: Option<String>,
	pub public_key_sig_key_id: Option<String>,
}

impl UserPublicKeyData
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
pub struct UserVerifyKeyData
{
	pub verify_key_pem: String,
	pub verify_key_alg: String,
	pub verify_key_id: SignKeyPairId,
}

impl UserVerifyKeyData
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
pub struct UserPublicKeyDataServerOutput
{
	pub public_key_id: EncryptionKeyPairId,
	pub public_key: String,
	pub public_key_alg: String,
	pub public_key_sig: Option<String>,
	pub public_key_sig_key_id: Option<String>,
}

impl UserPublicKeyDataServerOutput
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
pub struct UserVerifyKeyDataServerOutput
{
	pub verify_key_id: EncryptionKeyPairId,
	pub verify_key: String,
	pub verify_key_alg: String,
}

impl UserVerifyKeyDataServerOutput
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
pub struct PrepareLoginServerInput
{
	pub user_identifier: String,
}

impl PrepareLoginServerInput
{
	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}

	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}

#[derive(Serialize, Deserialize)]
pub struct PrepareLoginSaltServerOutput
{
	pub salt_string: String,
	pub derived_encryption_key_alg: String,
}

impl PrepareLoginSaltServerOutput
{
	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}

	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}

#[derive(Serialize, Deserialize)]
pub struct DoneLoginServerInput
{
	pub auth_key: String,
	pub device_identifier: String,
}

impl DoneLoginServerInput
{
	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}

	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DoneLoginServerOutput
{
	pub device_keys: DoneLoginServerKeysOutput,
	pub challenge: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize)]
pub enum DoneLoginServerReturn
{
	Otp,
	Direct(DoneLoginServerOutput),
}

//as base64 encoded string from the server
#[derive(Serialize, Deserialize, Clone)]
pub struct DoneLoginServerKeysOutput
{
	pub encrypted_master_key: String,
	pub encrypted_private_key: String,
	pub public_key_string: String,
	pub keypair_encrypt_alg: String,
	pub encrypted_sign_key: String,
	pub verify_key_string: String,
	pub keypair_sign_alg: String,
	pub keypair_encrypt_id: EncryptionKeyPairId,
	pub keypair_sign_id: SignKeyPairId,
	pub user_id: UserId,
	pub device_id: DeviceId,
	pub user_group_id: GroupId,
}

impl DoneLoginServerKeysOutput
{
	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}

	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}

#[derive(Serialize, Deserialize)]
pub struct VerifyLoginInput
{
	pub auth_key: String,
	pub device_identifier: String,
	pub challenge: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyLoginOutput
{
	pub user_keys: Vec<GroupKeyServerOutput>,
	pub hmac_keys: Vec<GroupHmacData>,
	pub jwt: String,
	pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyLoginLightOutput
{
	pub jwt: String,
	pub refresh_token: String,
}

/**
For refresh jwt
*/
#[derive(Serialize, Deserialize)]
pub struct DoneLoginLightServerOutput
{
	pub user_id: UserId,
	pub jwt: String,
	pub device_id: DeviceId,
}

#[derive(Serialize, Deserialize)]
pub struct UserInitServerOutput
{
	pub jwt: String,
	pub invites: Vec<GroupInviteReqList>,
}

#[derive(Serialize, Deserialize)]
pub struct JwtRefreshInput
{
	pub refresh_token: String,
}

impl JwtRefreshInput
{
	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}

	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}

#[derive(Serialize, Deserialize)]
pub struct PrepareLoginForKeyUpdateServerOutput
{
	pub client_random_value: String, //instead of prepare login (where the server creates the salt), for key update the client creates the salt for the old keys
	pub derived_encryption_key_alg: String,
	pub key_id: SymKeyId,
}

impl PrepareLoginForKeyUpdateServerOutput
{
	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}

	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateServerInput
{
	pub user_identifier: String,
}

impl UserUpdateServerInput
{
	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}

	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}
}

#[derive(Serialize, Deserialize)]
pub struct UserDeviceList
{
	pub device_id: String,
	pub time: u128,
	pub device_identifier: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserJwtInfo
{
	pub id: UserId,
	pub device_id: DeviceId,
}

#[derive(Serialize, Deserialize)]
pub struct Claims
{
	//jwt defaults
	pub aud: UserId,   //the user id
	pub sub: DeviceId, //the device id
	pub exp: usize,
	pub iat: usize,
	pub fresh: bool, //was this token from refresh jwt or from login
}

#[derive(Serialize, Deserialize)]
pub struct OtpRegister
{
	pub secret: String, //base32 endowed secret
	pub alg: &'static str,
	pub recover: [String; 6],
}

#[derive(Serialize, Deserialize)]
pub struct OtpInput
{
	pub token: String,
	pub auth_key: String,
	pub device_identifier: String,
}

#[derive(Serialize, Deserialize)]
pub struct OtpRecoveryKeysOutput
{
	pub keys: Vec<String>,
}
