/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CipherRequestModel {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::CipherType>,
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "favorite", skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
    #[serde(rename = "reprompt", skip_serializing_if = "Option::is_none")]
    pub reprompt: Option<crate::models::CipherRepromptType>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::CipherFieldModel>>,
    #[serde(rename = "passwordHistory", skip_serializing_if = "Option::is_none")]
    pub password_history: Option<Vec<crate::models::CipherPasswordHistoryModel>>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "attachments2", skip_serializing_if = "Option::is_none")]
    pub attachments2:
        Option<::std::collections::HashMap<String, crate::models::CipherAttachmentModel>>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<Box<crate::models::CipherLoginModel>>,
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<crate::models::CipherCardModel>>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<Box<crate::models::CipherIdentityModel>>,
    #[serde(rename = "secureNote", skip_serializing_if = "Option::is_none")]
    pub secure_note: Option<Box<crate::models::CipherSecureNoteModel>>,
    #[serde(
        rename = "lastKnownRevisionDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_known_revision_date: Option<String>,
}

impl CipherRequestModel {
    pub fn new(name: String) -> CipherRequestModel {
        CipherRequestModel {
            _type: None,
            organization_id: None,
            folder_id: None,
            favorite: None,
            reprompt: None,
            name,
            notes: None,
            fields: None,
            password_history: None,
            attachments: None,
            attachments2: None,
            login: None,
            card: None,
            identity: None,
            secure_note: None,
            last_known_revision_date: None,
        }
    }
}
