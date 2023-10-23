use chrono::{DateTime, Utc};

use crate::vault::{generate_totp, TotpResponse};

use super::client_vault::ClientVault;

impl<'a> ClientVault<'a> {
    /// Generate a TOTP code from a provided key.
    ///
    /// Key can be either:
    /// - A base32 encoded string
    /// - OTP Auth URI
    /// - Steam URI
    ///
    pub async fn generate_totp(&'a self, key: String, time: Option<DateTime<Utc>>) -> TotpResponse {
        generate_totp(key, time).await
    }
}
