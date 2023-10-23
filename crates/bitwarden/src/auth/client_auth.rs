use super::{
    password::{password_strength, satisfies_policy, MasterPasswordPolicyOptions},
    register::{make_register_keys, register},
    RegisterKeyResponse, RegisterRequest,
};
use crate::{client::kdf::Kdf, error::Result, Client};

pub struct ClientAuth<'a> {
    pub(crate) client: &'a mut crate::Client,
}

impl<'a> ClientAuth<'a> {
    pub async fn password_strength(
        &self,
        password: String,
        email: String,
        additional_inputs: Vec<String>,
    ) -> u8 {
        password_strength(password, email, additional_inputs)
    }

    pub async fn satisfies_policy(
        &self,
        password: String,
        strength: u8,
        policy: &MasterPasswordPolicyOptions,
    ) -> bool {
        satisfies_policy(password, strength, policy)
    }

    pub fn make_register_keys(
        &self,
        email: String,
        password: String,
        kdf: Kdf,
    ) -> Result<RegisterKeyResponse> {
        make_register_keys(email, password, kdf)
    }

    #[cfg(feature = "internal")]
    pub async fn register(&mut self, input: &RegisterRequest) -> Result<()> {
        register(self.client, input).await
    }
}

impl<'a> Client {
    pub fn auth(&'a mut self) -> ClientAuth<'a> {
        ClientAuth { client: self }
    }
}
