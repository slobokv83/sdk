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
pub struct IapCheckRequestModel {
    #[serde(rename = "paymentMethodType")]
    pub payment_method_type: crate::models::PaymentMethodType,
}

impl IapCheckRequestModel {
    pub fn new(payment_method_type: crate::models::PaymentMethodType) -> IapCheckRequestModel {
        IapCheckRequestModel {
            payment_method_type,
        }
    }
}
