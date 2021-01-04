use std::collections::HashMap;
use uuid::Uuid;

use super::prelude::*;
use crate::settings::{Password, Settings};

pub struct SessionService {
    sessions: RwLock<HashMap<SessionId, Session>>,
    session_cur_id: Mutex<SessionId>,
    session_start_id: u64,
    max_sessions: usize,
    session_idle_time: i64,
    session_password_settings: Password,
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
            session_cur_id: Mutex::new(settings.session.start_id),
            session_start_id: settings.session.start_id,
            max_sessions: settings.session.max_sessions,
            session_idle_time: settings.session.idle_time,
            session_password_settings: settings.session.password.clone(),
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

    pub async fn create_session(&self) -> Result<(SessionId, SessionPassword), SessionError> {
        let mut sessions = self.sessions.write().await;
        let mut cur_id_mtx = self.session_cur_id.lock().await;

        let mut next_free_id = || {
            // If no free sessions
            if sessions.len() >= self.max_sessions {
                return Err(SessionError::MaxSessions);
            }

            let mut fn_next_id = || {
                let next_id = *cur_id_mtx;

                // Update current id
                if *cur_id_mtx >= self.session_start_id + self.max_sessions as u64 {
                    *cur_id_mtx = self.session_start_id;
                } else {
                    *cur_id_mtx += 1;
                }

                next_id
            };

            let next_id = {
                let mut id = fn_next_id();
                while sessions.contains_key(&id) {
                    id = fn_next_id();
                }
                id
            };

            Ok(next_id)
        };

        let id = next_free_id()?;
        let password = generate_password(&self.session_password_settings)?;

        let session = Session {
            id: id.clone(),
            password: password.clone(),
            connections: Default::default(),
        };

        if sessions.contains_key(&id) {}
        sessions.insert(id.clone(), session);

        Ok((id, password))
    }
}

// fn next_id(
//     cur_id: SessionId,
//     start_id: SessionId,
//     max_sessions: usize,
// ) -> (
//     /* new_cur_id */ SessionId,
//     /* next_id */ SessionId,
// ) {
//     let next_id = cur_id;
//     let new_cur_id;
//
//     // Update current id
//     if cur_id >= start_id + max_sessions {
//         new_cur_id = start_id;
//     } else {
//         new_cur_id = cur_id + 1;
//     }
//
//     (new_cur_id, next_id)
// }
//
// fn next_free_id<S: ?Sized>(
//     sessions: RwLockWriteGuard<S>,
//     cur_id_mtx: MutexGuard<SessionId>,
//     start_id: SessionId,
//     max_sessions: usize,
// ) -> Result<SessionId, SessionError> {
//     // If no free sessions
//     if sessions.len() >= max_sessions {
//         return Err(SessionError::MaxSessions);
//     }
//
//     let next_id = || {
//         let next_id = *cur_id_mtx;
//
//         // Update current id
//         if cur_id_mtx >= start_id + max_sessions {
//             *cur_id_mtx = start_id;
//         } else {
//             *cur_id_mtx = cur_id + 1;
//         }
//
//         next_id
//     };
//
//     let next_id = {
//         let mut id = next_id();
//         while sessions.contains_key(&id) {
//             id = next_id();
//         }
//         id
//     };
//
//     Ok(next_id)
// }

fn generate_password(password_settings: &Password) -> Result<SessionPassword, SessionError> {
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
        .map_err(|err| SessionError::GeneratePasswordError(err.to_string()))
}

pub struct Connection {}

pub type SessionId = u64;
pub type SessionPassword = String;
pub type ClientId = Uuid;
