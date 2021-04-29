use crate::config;
use crate::TEST::domain::local_prelude::*;
use crate::TEST::port::auth::service::ClientId;
use crate::TEST::port::room::repo as room_repo;
use crate::TEST::port::room::repo::RoomRepo;
use crate::TEST::port::room::service::*;
use crate::TEST::port::{ServiceError, ServiceResult};

use std::cell::Cell;
use std::collections::HashMap;

const MIN_ROOM_ID: RoomId = 100_000;
const MAX_ROOM_ID: RoomId = 1_000_000;

#[inline]
fn max_rooms() -> usize {
    (MAX_ROOM_ID - MIN_ROOM_ID) as usize
}

pub struct RoomServiceImpl<R: RoomRepo> {
    cfg: config::Room,
    repo: Arc<R>,
    rooms: RwLock<HashMap<RoomId, Room>>,

    cur_id: Mutex<RoomId>,
}

struct Room {
    pub clients: HashMap<ClientId, Client>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            clients: Default::default(),
        }
    }
}

struct Client {}

impl<R: RoomRepo> RoomServiceImpl<R> {
    pub fn new(cfg: config::Room, repo: Arc<R>) -> Self {
        Self {
            cfg,
            repo,
            rooms: Default::default(),
            cur_id: Mutex::new(MIN_ROOM_ID),
        }
    }
}

#[async_trait::async_trait]
impl<R: RoomRepo> RoomService for RoomServiceImpl<R> {
    async fn create_room(&self, req: CreateRoomRequest) -> ServiceResult<CreateRoomResponse> {
        // Get new room id
        let room_id = {
            let rooms = self.rooms.read().await;
            let mut cur_id = self.cur_id.lock().await;
            next_free_id(&rooms, &mut cur_id)?
        };

        // Generate master password
        let master_password = generate_password(&self.cfg.password)?;

        let create_room_cred_req = room_repo::CreateRoomCredentialsRequest {
            room_id,
            master_password: (master_password, room_repo::RoomPasswordFeature::OneOff),
        };
        let create_room_cred_res = self
            .repo
            .create_room_credentials(create_room_cred_req)
            .await?;

        let room_cred: RoomCredentials = create_room_cred_res.room_cred.into();

        // Add new room
        self.rooms.write().await.insert(room_id, Room::new());

        let res = CreateRoomResponse { room_cred };

        Ok(res)
    }
}

fn next_free_id(rooms: &HashMap<RoomId, Room>, cur_id: &mut RoomId) -> ServiceResult<RoomId> {
    // If no free rooms
    if rooms.len() >= max_rooms() {
        return Err(ServiceError::CommonError(anyhow::anyhow!(
            "maximum number of rooms reached"
        )));
    }

    let mut id = next_id(*cur_id);
    while rooms.contains_key(&id) {
        id = next_id(*cur_id);
    }

    Ok(id)
}

#[inline]
fn next_id(cur_id: RoomId) -> RoomId {
    MIN_ROOM_ID + (cur_id - MIN_ROOM_ID + 1) % MAX_ROOM_ID
}

fn generate_password(password_settings: &config::Password) -> ServiceResult<String> {
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

    generator
        .generate_one()
        .map_err(|err| ServiceError::CommonError(anyhow::anyhow!(err)))
}

impl From<room_repo::RoomPasswordFeature> for RoomPasswordFeature {
    fn from(f: room_repo::RoomPasswordFeature) -> Self {
        match f {
            room_repo::RoomPasswordFeature::OneOff => RoomPasswordFeature::OneOff,
            room_repo::RoomPasswordFeature::Expiring { expires_in } => {
                RoomPasswordFeature::Expiring { expires_in }
            }
        }
    }
}

impl From<room_repo::RoomCredentials> for RoomCredentials {
    fn from(f: room_repo::RoomCredentials) -> Self {
        Self {
            id: f.id,
            passwords: f
                .passwords
                .into_iter()
                .map(|(p, f)| (p, f.into()))
                .collect(),
        }
    }
}
