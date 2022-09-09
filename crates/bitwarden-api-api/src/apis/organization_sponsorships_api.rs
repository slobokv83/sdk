/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`organization_sponsorship_redeem_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipRedeemPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_sponsored_sponsored_org_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipSponsoredSponsoredOrgIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_sponsored_sponsored_org_id_remove_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipSponsoredSponsoredOrgIdRemovePostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_sponsoring_org_id_families_for_enterprise_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipSponsoringOrgIdFamiliesForEnterprisePostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_sponsoring_org_id_families_for_enterprise_resend_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipSponsoringOrgIdFamiliesForEnterpriseResendPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_sponsoring_org_id_sync_status_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipSponsoringOrgIdSyncStatusGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_sponsoring_organization_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipSponsoringOrganizationIdDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_sponsoring_organization_id_delete_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipSponsoringOrganizationIdDeletePostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_sync_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipSyncPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organization_sponsorship_validate_token_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganizationSponsorshipValidateTokenPostError {
    UnknownValue(serde_json::Value),
}

pub async fn organization_sponsorship_redeem_post(
    configuration: &configuration::Configuration,
    sponsorship_token: Option<&str>,
    organization_sponsorship_redeem_request_model: Option<
        crate::models::OrganizationSponsorshipRedeemRequestModel,
    >,
) -> Result<(), Error<OrganizationSponsorshipRedeemPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/redeem",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sponsorship_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("sponsorshipToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder =
        local_var_req_builder.json(&organization_sponsorship_redeem_request_model);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<OrganizationSponsorshipRedeemPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_sponsored_sponsored_org_id_delete(
    configuration: &configuration::Configuration,
    sponsored_org_id: &str,
) -> Result<(), Error<OrganizationSponsorshipSponsoredSponsoredOrgIdDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/sponsored/{sponsoredOrgId}",
        local_var_configuration.base_path,
        sponsoredOrgId = crate::apis::urlencode(sponsored_org_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<OrganizationSponsorshipSponsoredSponsoredOrgIdDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_sponsored_sponsored_org_id_remove_post(
    configuration: &configuration::Configuration,
    sponsored_org_id: &str,
) -> Result<(), Error<OrganizationSponsorshipSponsoredSponsoredOrgIdRemovePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/sponsored/{sponsoredOrgId}/remove",
        local_var_configuration.base_path,
        sponsoredOrgId = crate::apis::urlencode(sponsored_org_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<
            OrganizationSponsorshipSponsoredSponsoredOrgIdRemovePostError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_sponsoring_org_id_families_for_enterprise_post(
    configuration: &configuration::Configuration,
    sponsoring_org_id: &str,
    organization_sponsorship_create_request_model: Option<
        crate::models::OrganizationSponsorshipCreateRequestModel,
    >,
) -> Result<(), Error<OrganizationSponsorshipSponsoringOrgIdFamiliesForEnterprisePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/{sponsoringOrgId}/families-for-enterprise",
        local_var_configuration.base_path,
        sponsoringOrgId = crate::apis::urlencode(sponsoring_org_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder =
        local_var_req_builder.json(&organization_sponsorship_create_request_model);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<
            OrganizationSponsorshipSponsoringOrgIdFamiliesForEnterprisePostError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_sponsoring_org_id_families_for_enterprise_resend_post(
    configuration: &configuration::Configuration,
    sponsoring_org_id: &str,
) -> Result<(), Error<OrganizationSponsorshipSponsoringOrgIdFamiliesForEnterpriseResendPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/{sponsoringOrgId}/families-for-enterprise/resend",
        local_var_configuration.base_path,
        sponsoringOrgId = crate::apis::urlencode(sponsoring_org_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<
            OrganizationSponsorshipSponsoringOrgIdFamiliesForEnterpriseResendPostError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_sponsoring_org_id_sync_status_get(
    configuration: &configuration::Configuration,
    sponsoring_org_id: &str,
) -> Result<(), Error<OrganizationSponsorshipSponsoringOrgIdSyncStatusGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/{sponsoringOrgId}/sync-status",
        local_var_configuration.base_path,
        sponsoringOrgId = crate::apis::urlencode(sponsoring_org_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<OrganizationSponsorshipSponsoringOrgIdSyncStatusGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_sponsoring_organization_id_delete(
    configuration: &configuration::Configuration,
    sponsoring_organization_id: &str,
) -> Result<(), Error<OrganizationSponsorshipSponsoringOrganizationIdDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/{sponsoringOrganizationId}",
        local_var_configuration.base_path,
        sponsoringOrganizationId = crate::apis::urlencode(sponsoring_organization_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<OrganizationSponsorshipSponsoringOrganizationIdDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_sponsoring_organization_id_delete_post(
    configuration: &configuration::Configuration,
    sponsoring_organization_id: &str,
) -> Result<(), Error<OrganizationSponsorshipSponsoringOrganizationIdDeletePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/{sponsoringOrganizationId}/delete",
        local_var_configuration.base_path,
        sponsoringOrganizationId = crate::apis::urlencode(sponsoring_organization_id.to_string())
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<
            OrganizationSponsorshipSponsoringOrganizationIdDeletePostError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_sync_post(
    configuration: &configuration::Configuration,
    organization_sponsorship_sync_request_model: Option<
        crate::models::OrganizationSponsorshipSyncRequestModel,
    >,
) -> Result<
    crate::models::OrganizationSponsorshipSyncResponseModel,
    Error<OrganizationSponsorshipSyncPostError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/sync",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder =
        local_var_req_builder.json(&organization_sponsorship_sync_request_model);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OrganizationSponsorshipSyncPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn organization_sponsorship_validate_token_post(
    configuration: &configuration::Configuration,
    sponsorship_token: Option<&str>,
) -> Result<bool, Error<OrganizationSponsorshipValidateTokenPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/organization/sponsorship/validate-token",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sponsorship_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("sponsorshipToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OrganizationSponsorshipValidateTokenPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
