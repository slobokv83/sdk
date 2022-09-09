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
pub struct Group {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

impl Group {
    pub fn new(name: String, external_id: String) -> Group {
        Group {
            name,
            external_id,
            users: None,
        }
    }
}
