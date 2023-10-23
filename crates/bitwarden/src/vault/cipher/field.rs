use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use super::linked_id::LinkedIdType;
use crate::{
    client::encryption_settings::EncryptionSettings,
    crypto::{Decryptable, EncString, Encryptable},
    error::Result,
};

#[derive(Clone, Copy, Serialize_repr, Deserialize_repr, Debug, JsonSchema)]
#[repr(u8)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum FieldType {
    Text = 0,
    Hidden = 1,
    Boolean = 2,
    Linked = 3,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct Field {
    name: Option<EncString>,
    value: Option<EncString>,
    r#type: FieldType,

    linked_id: Option<LinkedIdType>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct FieldView {
    name: Option<String>,
    value: Option<String>,
    r#type: FieldType,

    linked_id: Option<LinkedIdType>,
}

impl Encryptable<Field> for FieldView {
    fn encrypt(self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<Field> {
        Ok(Field {
            name: self.name.encrypt(enc, org_id)?,
            value: self.value.encrypt(enc, org_id)?,
            r#type: self.r#type,
            linked_id: self.linked_id,
        })
    }
}

impl Decryptable<FieldView> for Field {
    fn decrypt(&self, enc: &EncryptionSettings, org_id: &Option<Uuid>) -> Result<FieldView> {
        Ok(FieldView {
            name: self.name.decrypt(enc, org_id)?,
            value: self.value.decrypt(enc, org_id)?,
            r#type: self.r#type,
            linked_id: self.linked_id,
        })
    }
}
