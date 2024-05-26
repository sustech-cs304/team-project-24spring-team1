use std::collections::HashMap;
use tokio::sync::oneshot::Sender;
use uuid::Uuid;

use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
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
        if let Some(tx) = self.subscribers.remove(&identifier).flatten() {
            tx.send(result).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::oneshot::channel;

    lazy_static! {
        static ref AUTH_RESULT: AuthResult = AuthResult {
            sustech_id: 11111111,
            name: "Elaina".to_string(),
            email: "elaina@sustech.edu.cn".to_string(),
        };
    }

    #[tokio::test]
    async fn test_basic() {
        let mut provider = CRAProvider::new();
        let (tx, rx) = channel();

        let identifier = provider.alloc_identifier();
        provider.subscribe(identifier, tx).unwrap();

        provider.callback(identifier, AUTH_RESULT.clone()).unwrap();
        assert_eq!(rx.await, Ok(AUTH_RESULT.clone()));
    }

    #[tokio::test]
    async fn test_unsubscribe() {
        let mut provider = CRAProvider::new();
        let (tx, rx) = channel();

        let identifier = provider.alloc_identifier();
        provider.subscribe(identifier, tx).unwrap();
        provider.unsubscribe(identifier).unwrap();

        let result = rx.await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("channel closed"));
    }

    #[tokio::test]
    async fn test_invalid() {
        let mut provider = CRAProvider::new();
        let (tx, _rx) = channel();

        let identifier = Uuid::new_v4();
        provider.subscribe(identifier, tx).unwrap_err();
        provider
            .callback(identifier, AUTH_RESULT.clone())
            .unwrap_err();
    }
}
