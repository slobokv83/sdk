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
pub struct ImportOrganizationUsersRequestModel {
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::Group>>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
    #[serde(rename = "overwriteExisting", skip_serializing_if = "Option::is_none")]
    pub overwrite_existing: Option<bool>,
    #[serde(rename = "largeImport", skip_serializing_if = "Option::is_none")]
    pub large_import: Option<bool>,
}

impl ImportOrganizationUsersRequestModel {
    pub fn new() -> ImportOrganizationUsersRequestModel {
        ImportOrganizationUsersRequestModel {
            groups: None,
            users: None,
            overwrite_existing: None,
            large_import: None,
        }
    }
}
