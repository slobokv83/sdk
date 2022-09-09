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
pub struct VerifyEmailRequestModel {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "token")]
    pub token: String,
}

impl VerifyEmailRequestModel {
    pub fn new(user_id: String, token: String) -> VerifyEmailRequestModel {
        VerifyEmailRequestModel { user_id, token }
    }
}
