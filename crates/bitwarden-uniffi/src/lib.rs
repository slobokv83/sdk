uniffi::setup_scaffolding!();

use std::sync::Arc;

use async_lock::RwLock;
use auth::ClientAuth;
use bitwarden::{client::client_settings::ClientSettings, mobile::crypto::InitCryptoRequest};

pub mod auth;
mod error;
pub mod tool;
mod uniffi_support;
pub mod vault;

#[cfg(feature = "docs")]
pub mod docs;

use error::Result;
use tool::ClientGenerators;
use vault::ClientVault;

#[derive(uniffi::Object)]
pub struct Client(RwLock<bitwarden::Client>);

#[derive(uniffi::Object)]
pub struct ClientCrypto(Arc<Client>);

#[uniffi::export]
impl Client {
    /// Initialize a new instance of the SDK client
    #[uniffi::constructor]
    pub fn new(settings: Option<ClientSettings>) -> Arc<Self> {
        Arc::new(Self(RwLock::new(bitwarden::Client::new(settings))))
    }

    /// Crypto operations
    pub fn crypto(self: Arc<Self>) -> Arc<ClientCrypto> {
        Arc::new(ClientCrypto(self))
    }

    /// Vault item operations
    pub fn vault(self: Arc<Self>) -> Arc<ClientVault> {
        Arc::new(ClientVault(self))
    }

    /// Generator operations
    pub fn generators(self: Arc<Self>) -> Arc<ClientGenerators> {
        Arc::new(ClientGenerators(self))
    }

    /// Auth operations
    pub fn auth(self: Arc<Self>) -> Arc<ClientAuth> {
        Arc::new(ClientAuth(self))
    }

    /// Test method, echoes back the input
    pub fn echo(&self, msg: String) -> String {
        msg
    }
}

#[uniffi::export]
impl ClientCrypto {
    /// Initialization method for the crypto. Needs to be called before any other crypto operations.
    pub async fn initialize_crypto(&self, req: InitCryptoRequest) -> Result<()> {
        Ok(self
            .0
             .0
            .write()
            .await
            .crypto()
            .initialize_crypto(req)
            .await?)
    }
}
