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
pub struct CipherFieldModel {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::FieldType>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "linkedId", skip_serializing_if = "Option::is_none")]
    pub linked_id: Option<i32>,
}

impl CipherFieldModel {
    pub fn new() -> CipherFieldModel {
        CipherFieldModel {
            _type: None,
            name: None,
            value: None,
            linked_id: None,
        }
    }
}
