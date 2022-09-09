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
pub struct ProviderUserBulkRequestModel {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
}

impl ProviderUserBulkRequestModel {
    pub fn new(ids: Vec<String>) -> ProviderUserBulkRequestModel {
        ProviderUserBulkRequestModel { ids }
    }
}
