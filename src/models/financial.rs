/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Financial {
    #[serde(rename = "field_name", skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "custom_key", skip_serializing_if = "Option::is_none")]
    pub custom_key: Option<String>,
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<crate::models::FinancialAccounts>>,
    #[serde(rename = "requirement", skip_serializing_if = "Option::is_none")]
    pub requirement: Option<crate::models::DocumentsRequired>,
}

impl Financial {
    pub fn new() -> Financial {
        Financial {
            field_name: None,
            custom_key: None,
            accounts: None,
            requirement: None,
        }
    }
}


