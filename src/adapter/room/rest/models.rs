use crate::adapter::rest_prelude::*;
use crate::port::room::service as room_service;

use actix::prelude::*;
use std::collections::HashMap;

pub type RoomId = room_service::RoomId;
pub type FileId = room_service::FileId;
pub type ClientId = room_service::ClientId;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateRoomResponse {
    pub room_id: RoomId,
    pub master_password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ConnectRoomPathRequest {
    pub room_id: RoomId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DisconnectRoomPathRequest {
    pub room_id: RoomId,
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GetFilesPathRequest {
    pub room_id: RoomId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GetFilesResponse {
    pub files: HashMap<FileId, File>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct WsConnPathRequest {
    pub room_id: RoomId,
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
