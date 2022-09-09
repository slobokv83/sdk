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
pub struct DeviceResponseModelListResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::DeviceResponseModel>>,
    #[serde(rename = "continuationToken", skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

impl DeviceResponseModelListResponseModel {
    pub fn new() -> DeviceResponseModelListResponseModel {
        DeviceResponseModelListResponseModel {
            object: None,
            data: None,
            continuation_token: None,
        }
    }
}
