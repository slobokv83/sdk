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
pub struct PaymentResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "userProfile", skip_serializing_if = "Option::is_none")]
    pub user_profile: Option<Box<crate::models::ProfileResponseModel>>,
    #[serde(
        rename = "paymentIntentClientSecret",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_intent_client_secret: Option<String>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl PaymentResponseModel {
    pub fn new() -> PaymentResponseModel {
        PaymentResponseModel {
            object: None,
            user_profile: None,
            payment_intent_client_secret: None,
            success: None,
        }
    }
}
