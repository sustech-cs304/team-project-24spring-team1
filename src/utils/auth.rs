use std::collections::HashMap;
use tokio::sync::oneshot::Sender;
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct AuthResult {
    pub sustech_id: i32,
    pub email: String,
    pub name: String,
}

pub trait AuthProvider: std::fmt::Debug {
    fn alloc_identifier(&mut self) -> Uuid;

    fn subscribe(&mut self, identifier: Uuid, tx: Sender<AuthResult>) -> Result<()>;
    fn unsubscribe(&mut self, identifier: Uuid) -> Result<()>;
    fn callback(&mut self, identifier: Uuid, result: AuthResult) -> Result<()>;
}

// ===== Providers =====

#[derive(Debug, Default)]
pub struct CRAProvider {
    subscribers: HashMap<Uuid, Option<Sender<AuthResult>>>,
}

impl CRAProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn check_identifier_exists(&self, identifier: Uuid) -> Result<()> {
        if !self.subscribers.contains_key(&identifier) {
            return Err(Error::NotAcceptable("Invalid identifier".to_owned()));
        }
        Ok(())
    }
}

impl AuthProvider for CRAProvider {
    fn alloc_identifier(&mut self) -> Uuid {
        let uuid = std::iter::repeat_with(Uuid::new_v4)
            .find(|id| !self.subscribers.contains_key(id))
            .unwrap();
        self.subscribers.insert(uuid, None);
        uuid
    }

    fn subscribe(&mut self, identifier: Uuid, tx: Sender<AuthResult>) -> Result<()> {
        self.check_identifier_exists(identifier)?;
        self.subscribers.insert(identifier, Some(tx));
        Ok(())
    }

    fn unsubscribe(&mut self, identifier: Uuid) -> Result<()> {
        self.check_identifier_exists(identifier)?;
        self.subscribers.insert(identifier, None);
        Ok(())
    }

    fn callback(&mut self, identifier: Uuid, result: AuthResult) -> Result<()> {
        self.check_identifier_exists(identifier)?;
        let tx = self.subscribers.remove(&identifier).flatten().unwrap();
        tx.send(result).unwrap();
        Ok(())
    }
}
