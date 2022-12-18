//! # Sentc common to communicate with the backend api
//!
//! The input and output for and from the server. The server output and input are json strings.
//!
//! Contains type def for the ids. Ids are used internally.
//! A UserId is not the user identifier which is sued to login.
//!
//! Non std with alloc

#![no_std]

extern crate alloc;

use alloc::string::String;

pub mod content;
pub mod crypto;
pub mod file;
pub mod group;
pub mod server_default;
pub mod user;

pub use server_default::ServerOutput;

type GeneralIdFormat = String;
pub type GroupId = GeneralIdFormat;
pub type UserId = GeneralIdFormat;
pub type DeviceId = GeneralIdFormat;
pub type EncryptionKeyPairId = GeneralIdFormat;
pub type SignKeyPairId = GeneralIdFormat;
pub type SymKeyId = GeneralIdFormat;
pub type ContentId = GeneralIdFormat;
pub type CategoryId = GeneralIdFormat;

pub type FileId = GeneralIdFormat;
pub type PartId = GeneralIdFormat;
pub type FileSessionId = GeneralIdFormat;

pub type CustomerId = GeneralIdFormat;
pub type CustomerSecretTokenId = GeneralIdFormat;
pub type CustomerPublicTokenId = GeneralIdFormat;
pub type AppId = GeneralIdFormat;
pub type JwtKeyId = GeneralIdFormat;
