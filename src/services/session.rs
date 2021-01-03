use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::prelude::*;
use crate::settings::Settings;

pub struct SessionService {
    sessions: RwLock<HashMap<SessionId, Session>>,
    session_idle_time: i64,
}

pub struct Session {
    pub id: SessionId,
    pub password: SessionPassword,
    pub connections: HashMap<ClientId, Connection>,
}

impl SessionService {
    pub fn new(settings: &Settings) -> SessionService {
        SessionService {
            sessions: Default::default(),
            session_idle_time: settings.session.idle_time,
        }
    }

    pub async fn validate_password(
        &self,
        session_id: SessionId,
        session_password: SessionPassword,
    ) -> Result<(), SessionError> {
        self.sessions
            .read()
            .await
            .get(&session_id)
            .ok_or_else(|| SessionError::SessionNotFound(session_id))
            .and_then(|session| {
                if session.password == session_password {
                    Ok(())
                } else {
                    Err(SessionError::WrongPassword(session_id))
                }
            })
    }
}

pub struct Connection {}

pub type SessionId = u32;
pub type SessionPassword = String;
pub type ClientId = Uuid;
