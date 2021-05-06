use crate::config;
use crate::domain::local_prelude::*;
use crate::port::room::repo as room_repo;
use crate::port::room::repo::RoomRepo;
use crate::port::room::service::*;
use crate::port::{ServiceError, ServiceResult};

pub struct RoomServiceImpl<R: RoomRepo> {
    cfg: config::Room,
    repo: Arc<R>,
}

impl<R: RoomRepo> RoomServiceImpl<R> {
    pub fn new(cfg: config::Room, repo: Arc<R>) -> Self {
        Self { cfg, repo }
    }
}

#[async_trait::async_trait]
impl<R: RoomRepo> RoomService for RoomServiceImpl<R> {
    async fn create_room(&self, _: CreateRoomRequest) -> ServiceResult<CreateRoomResponse> {
        // Generate master password
        let master_password = generate_password(&self.cfg.password)?;

        let create_room_req = room_repo::CreateRoomRequest {
            client_ids: Default::default(),
            room_passwords: vec![(master_password, room_repo::RoomPasswordFeature::OneOff)]
                .into_iter()
                .collect(),
        };
        let create_room_res = self.repo.create_room(create_room_req).await?;

        let res = CreateRoomResponse {
            room_id: create_room_res.room_id,
            room_cred: create_room_res.room_cred.into(),
        };

        Ok(res)
    }

    async fn connect_room(&self, req: ConnectRoomRequest) -> ServiceResult<()> {
        let add_client_req = room_repo::AddClientRequest {
            room_id: req.room_id,
            client_id: req.client_id,
        };
        self.repo.add_client(add_client_req).await?;

        Ok(())
    }

    async fn disconnect_room(&self, req: DisconnectRoomRequest) -> ServiceResult<()> {
        let delete_client_req = room_repo::DeleteClientRequest {
            room_id: req.room_id,
            client_id: req.client_id,
        };
        self.repo.delete_client(delete_client_req).await?;

        Ok(())
    }
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
            passwords: f
                .passwords
                .into_iter()
                .map(|(p, f)| (p, f.into()))
                .collect(),
        }
    }
}
