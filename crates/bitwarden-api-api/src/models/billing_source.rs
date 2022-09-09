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
pub struct BillingSource {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::PaymentMethodType>,
    #[serde(rename = "cardBrand", skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "needsVerification", skip_serializing_if = "Option::is_none")]
    pub needs_verification: Option<bool>,
}

impl BillingSource {
    pub fn new() -> BillingSource {
        BillingSource {
            _type: None,
            card_brand: None,
            description: None,
            needs_verification: None,
        }
    }
}
