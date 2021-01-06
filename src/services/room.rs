use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

use super::prelude::*;
use crate::settings;
use crate::settings::Settings;

pub struct RoomService {
    rooms: RwLock<HashMap<RoomId, Room>>,
    settings: settings::Room,
    session_cur_id: Mutex<RoomId>,
}

struct Room {
    pub id: RoomId,
    pub invitations: Vec<RoomPassword>,
}

impl RoomService {
    pub fn new(settings: &Settings) -> RoomService {
        RoomService {
            rooms: Default::default(),
            settings: settings.room.clone(),
            session_cur_id: Mutex::new(settings.room.start_id),
        }
    }

    pub async fn connect(
        &self,
        client_id: ClientId,
        room_id: RoomId,
        room_password: RoomPassword,
    ) -> Result<(), RoomError> {
        let mut rooms = self.rooms.write().await;

        let room = rooms.get_mut(&room_id).ok_or_else(|| {
            RoomError::ConnectError(format!("room not found, room_id={}", room_id))
        })?;

        // Find and remove password
        room.invitations
            .iter()
            .position(|passw| *passw == room_password)
            .ok_or_else(|| {
                RoomError::ConnectError(format!("invalid password, room_id={}", room_id))
            })
            .map(|idx| {
                room.invitations.remove(idx);
            })?;

        // TODO: add client and broadcast

        Ok(())
    }

    pub async fn disconnect(&self, client_id: ClientId, room_id: RoomId) -> Result<(), RoomError> {
        let mut rooms = self.rooms.write().await;

        let room = rooms.get_mut(&room_id).ok_or_else(|| {
            RoomError::DisconnectError(format!("room not found, room_id={}", room_id))
        })?;

        // TODO: remove client and broadcast

        Ok(())
    }

    pub async fn create_session(&self) -> Result<(RoomId, RoomPassword), RoomError> {
        let mut rooms = self.rooms.write().await;
        let mut cur_id_mtx = self.session_cur_id.lock().await;

        let room_id = next_free_id(
            rooms.deref(),
            cur_id_mtx.deref_mut(),
            self.settings.start_id,
            self.settings.max_rooms,
        )
        .map_err(|err| RoomError::CreateRoomError(err.to_string()))?;

        let password = generate_password(&self.settings.password)
            .map_err(|err| RoomError::CreateRoomError(err.to_string()))?;

        let room = Room {
            id: room_id,
            invitations: vec![password.clone()],
        };

        rooms.insert(room_id, room);

        Ok((room_id, password))
    }
}

fn next_id(cur_id: &mut RoomId, start_id: RoomId, max_rooms: usize) -> RoomId {
    let next_id = *cur_id;

    // Update current id
    if *cur_id >= start_id + max_rooms as u64 {
        *cur_id = start_id;
    } else {
        *cur_id += 1;
    }

    next_id
}

fn next_free_id(
    rooms: &HashMap<RoomId, Room>,
    cur_id: &mut RoomId,
    start_id: RoomId,
    max_rooms: usize,
) -> Result<RoomId, &'static str> {
    // If no free rooms
    if rooms.len() >= max_rooms {
        return Err("maximum number of rooms reached");
    }

    let mut id = next_id(cur_id, start_id, max_rooms);
    while rooms.contains_key(&id) {
        id = next_id(cur_id, start_id, max_rooms);
    }

    Ok(id)
}

pub type RoomId = u64;
pub type RoomPassword = String;
pub type ClientId = Uuid;

fn generate_password(password_settings: &settings::Password) -> Result<RoomPassword, &'static str> {
    let generator = passwords::PasswordGenerator {
        length: password_settings.length,
        numbers: password_settings.use_numbers,
        lowercase_letters: password_settings.use_lowercase_letters,
        uppercase_letters: password_settings.use_uppercase_letters,
        symbols: password_settings.use_symbols,
        spaces: password_settings.use_spaces,
        exclude_similar_characters: password_settings.use_exclude_similar_characters,
        strict: password_settings.strict,
    };

    generator.generate_one()
}
