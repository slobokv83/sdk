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
pub struct SelectionReadOnlyResponseModel {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "hidePasswords", skip_serializing_if = "Option::is_none")]
    pub hide_passwords: Option<bool>,
}

impl SelectionReadOnlyResponseModel {
    pub fn new() -> SelectionReadOnlyResponseModel {
        SelectionReadOnlyResponseModel {
            id: None,
            read_only: None,
            hide_passwords: None,
        }
    }
}
