use std::collections::HashMap;
use chrono::{Duration, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Session {
    user_id: String,
    expires_at: i64,
}

#[derive(Clone, Debug)]
pub struct SessionStorage {
    sessions: HashMap<String, Session>,
}

impl SessionStorage {
    pub fn new() -> SessionStorage {
        Self { sessions: HashMap::new() }
    }
    pub fn create_session(&mut self, user_id: String) -> String {
        let session_id = Uuid::new_v4().to_string();
        let expires_at = (Utc::now() + Duration::hours(1)).timestamp();
        self.sessions.insert(session_id.clone(), Session { user_id, expires_at });
        session_id
    }
    pub fn validate_session(&self, session_id: &'static str) -> Option<&Session> {
        if let Some(session) = self.sessions.get(session_id) {
            if session.expires_at > Utc::now().timestamp() {
                return Some(session);
            }
        }
        None
    }

    pub fn refresh_session(&mut self, session_id: &'static str) -> Option<String> {
        let session_opt = self.sessions.get(session_id);
        if let Some(session) = session_opt {
            let new_session_id = Uuid::new_v4().to_string();
            let expires_at = (Utc::now() + Duration::hours(1)).timestamp();
            self.sessions.insert(new_session_id.clone(), Session {
                user_id: session.user_id.clone(),
                expires_at,
            });
            return Some(new_session_id);
        }
        None
    }
    pub fn cleanup_sessions(&mut self) {
        let now = Utc::now().timestamp();
        self.sessions.retain(|_, session| session.expires_at > now);
    }
}