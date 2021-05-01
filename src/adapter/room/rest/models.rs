use crate::port::room::service as room_service;
use std::convert::TryFrom;

pub type RoomId = u64;

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
                    f.room_cred.id
                ))
            }
            Some((p, _)) => p,
        };

        Ok(Self {
            room_id: f.room_cred.id,
            master_password,
        })
    }
}
