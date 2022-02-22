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


/// struct for typed errors of method [`get_data_provider_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDataProviderByIdError {
    Status500(),
    Status400(crate::models::ProblemDetails),
    Status404(crate::models::ProblemDetails),
    DefaultResponse(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_data_providers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDataProvidersError {
    Status500(),
    Status400(crate::models::ProblemDetails),
    DefaultResponse(crate::models::ProblemDetails),
    UnknownValue(serde_json::Value),
}


pub async fn get_data_provider_by_id(configuration: &configuration::Configuration, provider_id: &str) -> Result<crate::models::DataProvider, Error<GetDataProviderByIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/data-providers/{providerId}", local_var_configuration.base_path, providerId=crate::apis::urlencode(provider_id));
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
        let local_var_entity: Option<GetDataProviderByIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_data_providers(configuration: &configuration::Configuration, account_type: Option<&str>, document_type: Option<&str>, organization_category: Option<&str>, page_no: Option<i32>, page_size: Option<i32>, country: Option<&str>) -> Result<crate::models::DataProviderPaginatedList, Error<GetDataProvidersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/data-providers", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = account_type {
        local_var_req_builder = local_var_req_builder.query(&[("accountType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = document_type {
        local_var_req_builder = local_var_req_builder.query(&[("documentType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = organization_category {
        local_var_req_builder = local_var_req_builder.query(&[("organizationCategory", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_no {
        local_var_req_builder = local_var_req_builder.query(&[("pageNo", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = country {
        local_var_req_builder = local_var_req_builder.query(&[("country", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetDataProvidersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

