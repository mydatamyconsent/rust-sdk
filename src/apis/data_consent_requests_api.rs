/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`cancel_individual_data_consent_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelIndividualDataConsentRequestError {
    Status500(serde_json::Value),
    Status404(serde_json::Value),
    Status400(serde_json::Value),
    DefaultResponse(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cancel_organization_data_consent_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelOrganizationDataConsentRequestError {
    Status500(serde_json::Value),
    Status404(serde_json::Value),
    Status400(serde_json::Value),
    DefaultResponse(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_individual_data_consent_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIndividualDataConsentRequestError {
    Status500(serde_json::Value),
    Status404(serde_json::Value),
    Status400(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_organization_data_consent_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrganizationDataConsentRequestError {
    Status500(serde_json::Value),
    Status404(serde_json::Value),
    Status400(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_consent_requests_to_individuals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllConsentRequestsToIndividualsError {
    Status400(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_consent_requests_to_organizations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllConsentRequestsToOrganizationsError {
    Status400(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_individual_consent_request_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIndividualConsentRequestByIdError {
    Status404(serde_json::Value),
    Status400(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_organization_consent_request_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrganizationConsentRequestByIdError {
    Status404(serde_json::Value),
    Status400(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}


pub async fn cancel_individual_data_consent_request(configuration: &configuration::Configuration, request_id: &str) -> Result<(), Error<CancelIndividualDataConsentRequestError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/consent-requests/individual/{requestId}/cancel", local_var_configuration.base_path, requestId=crate::apis::urlencode(request_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CancelIndividualDataConsentRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn cancel_organization_data_consent_request(configuration: &configuration::Configuration, request_id: &str) -> Result<(), Error<CancelOrganizationDataConsentRequestError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/consent-requests/organization/{requestId}/cancel", local_var_configuration.base_path, requestId=crate::apis::urlencode(request_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CancelOrganizationDataConsentRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create data consent request for an individual.
pub async fn create_individual_data_consent_request(configuration: &configuration::Configuration, create_data_consent_request: crate::models::CreateDataConsentRequest) -> Result<crate::models::IndividualDataConsentRequestDetails, Error<CreateIndividualDataConsentRequestError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/consent-requests/individual", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_data_consent_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateIndividualDataConsentRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create data consent request for an organization.
pub async fn create_organization_data_consent_request(configuration: &configuration::Configuration, create_data_consent_request: crate::models::CreateDataConsentRequest) -> Result<crate::models::OrganizationDataConsentRequestDetails, Error<CreateOrganizationDataConsentRequestError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/consent-requests/organization", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_data_consent_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateOrganizationDataConsentRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_all_consent_requests_to_individuals(configuration: &configuration::Configuration, status: Option<crate::models::DataConsentStatus>, start_date_time: Option<String>, end_date_time: Option<String>, page_no: Option<i32>, page_size: Option<i32>) -> Result<crate::models::IndividualDataConsentRequestDetailsPaginatedList, Error<GetAllConsentRequestsToIndividualsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/consent-requests/individuals", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_date_time {
        local_var_req_builder = local_var_req_builder.query(&[("startDateTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_date_time {
        local_var_req_builder = local_var_req_builder.query(&[("endDateTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_no {
        local_var_req_builder = local_var_req_builder.query(&[("pageNo", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllConsentRequestsToIndividualsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_all_consent_requests_to_organizations(configuration: &configuration::Configuration, status: Option<crate::models::DataConsentStatus>, start_date_time: Option<String>, end_date_time: Option<String>, page_no: Option<i32>, page_size: Option<i32>) -> Result<crate::models::OrganizationDataConsentRequestDetailsPaginatedList, Error<GetAllConsentRequestsToOrganizationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/consent-requests/organizations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_date_time {
        local_var_req_builder = local_var_req_builder.query(&[("startDateTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_date_time {
        local_var_req_builder = local_var_req_builder.query(&[("endDateTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_no {
        local_var_req_builder = local_var_req_builder.query(&[("pageNo", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllConsentRequestsToOrganizationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_individual_consent_request_by_id(configuration: &configuration::Configuration, request_id: &str) -> Result<crate::models::DataConsentRequest, Error<GetIndividualConsentRequestByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/consent-requests/individuals/{requestId}", local_var_configuration.base_path, requestId=crate::apis::urlencode(request_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetIndividualConsentRequestByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_organization_consent_request_by_id(configuration: &configuration::Configuration, request_id: &str) -> Result<crate::models::DataConsentRequest, Error<GetOrganizationConsentRequestByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/consent-requests/organizations/{requestId}", local_var_configuration.base_path, requestId=crate::apis::urlencode(request_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetOrganizationConsentRequestByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

