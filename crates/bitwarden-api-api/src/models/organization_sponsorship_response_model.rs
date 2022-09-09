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
pub struct OrganizationSponsorshipResponseModel {
    #[serde(
        rename = "sponsoringOrganizationUserId",
        skip_serializing_if = "Option::is_none"
    )]
    pub sponsoring_organization_user_id: Option<String>,
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "offeredToEmail", skip_serializing_if = "Option::is_none")]
    pub offered_to_email: Option<String>,
    #[serde(
        rename = "planSponsorshipType",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_sponsorship_type: Option<crate::models::PlanSponsorshipType>,
    #[serde(rename = "lastSyncDate", skip_serializing_if = "Option::is_none")]
    pub last_sync_date: Option<String>,
    #[serde(rename = "validUntil", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
    #[serde(rename = "toDelete", skip_serializing_if = "Option::is_none")]
    pub to_delete: Option<bool>,
    #[serde(
        rename = "cloudSponsorshipRemoved",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_sponsorship_removed: Option<bool>,
}

impl OrganizationSponsorshipResponseModel {
    pub fn new() -> OrganizationSponsorshipResponseModel {
        OrganizationSponsorshipResponseModel {
            sponsoring_organization_user_id: None,
            friendly_name: None,
            offered_to_email: None,
            plan_sponsorship_type: None,
            last_sync_date: None,
            valid_until: None,
            to_delete: None,
            cloud_sponsorship_removed: None,
        }
    }
}
