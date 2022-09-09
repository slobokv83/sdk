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
pub struct RevokeAccessTokensRequest {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

impl RevokeAccessTokensRequest {
    pub fn new(ids: Vec<String>) -> RevokeAccessTokensRequest {
        RevokeAccessTokensRequest { ids }
    }
}
