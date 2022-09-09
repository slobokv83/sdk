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
pub struct VerifyOtpRequestModel {
    #[serde(rename = "otp")]
    pub otp: String,
}

impl VerifyOtpRequestModel {
    pub fn new(otp: String) -> VerifyOtpRequestModel {
        VerifyOtpRequestModel { otp }
    }
}
