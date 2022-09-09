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
pub struct OrganizationUserResetPasswordEnrollmentRequestModel {
    #[serde(rename = "masterPasswordHash", skip_serializing_if = "Option::is_none")]
    pub master_password_hash: Option<String>,
    #[serde(rename = "otp", skip_serializing_if = "Option::is_none")]
    pub otp: Option<String>,
    #[serde(
        rename = "authRequestAccessCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_request_access_code: Option<String>,
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "resetPasswordKey", skip_serializing_if = "Option::is_none")]
    pub reset_password_key: Option<String>,
}

impl OrganizationUserResetPasswordEnrollmentRequestModel {
    pub fn new() -> OrganizationUserResetPasswordEnrollmentRequestModel {
        OrganizationUserResetPasswordEnrollmentRequestModel {
            master_password_hash: None,
            otp: None,
            auth_request_access_code: None,
            secret: None,
            reset_password_key: None,
        }
    }
}
