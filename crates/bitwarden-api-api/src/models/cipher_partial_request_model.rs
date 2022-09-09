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
pub struct CipherPartialRequestModel {
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "favorite", skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
}

impl CipherPartialRequestModel {
    pub fn new() -> CipherPartialRequestModel {
        CipherPartialRequestModel {
            folder_id: None,
            favorite: None,
        }
    }
}
