use crate::adapter::rest_prelude::*;
use crate::port::room::service as room_service;

use actix::prelude::*;
use std::convert::TryFrom;

pub type RoomId = room_service::RoomId;
pub type FileId = room_service::FileId;
pub type ClientId = room_service::ClientId;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateRoomResponse {
    pub room_id: RoomId,
    pub master_password: String,
}

impl TryFrom<room_service::CreateRoomResponse> for CreateRoomResponse {
    type Error = anyhow::Error;

    fn try_from(f: room_service::CreateRoomResponse) -> Result<Self, Self::Error> {
        let master_password = match f.room_cred.passwords.into_iter().next() {
            None => {
                return Err(anyhow::anyhow!(
                    "no master password in room id={}",
                    f.room_id
                ))
            }
            Some((p, _)) => p,
        };

        Ok(Self {
            room_id: f.room_id,
            master_password,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddFilePathRequest {
    pub room_id: RoomId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddFileBodyRequest {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AddFileResponse {
    pub file: File,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct File {
    pub id: FileId,
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub source_client_id: ClientId,
}

impl From<room_service::File> for File {
    fn from(f: room_service::File) -> Self {
        Self {
            id: f.id,
            name: f.name,
            size: f.size,
            mime_type: f.mime_type,
            source_client_id: f.source_client_id,
        }
    }
}

#[derive(Debug, Message)]
#[rtype(result = "Result<(), ApiError>")]
#[repr(transparent)]
pub struct FilePart(pub actix_web::web::Bytes);
