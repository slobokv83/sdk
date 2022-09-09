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
pub struct BitPayInvoiceRequestModel {
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "credit", skip_serializing_if = "Option::is_none")]
    pub credit: Option<bool>,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "returnUrl", skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl BitPayInvoiceRequestModel {
    pub fn new(amount: f64) -> BitPayInvoiceRequestModel {
        BitPayInvoiceRequestModel {
            user_id: None,
            organization_id: None,
            credit: None,
            amount,
            return_url: None,
            name: None,
            email: None,
        }
    }
}
