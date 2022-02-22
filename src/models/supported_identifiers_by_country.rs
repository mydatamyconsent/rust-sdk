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
pub struct SupportedIdentifiersByCountry {
    #[serde(rename = "iso2", skip_serializing_if = "Option::is_none")]
    pub iso2: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "flag", skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    #[serde(rename = "individualIdentifiers", skip_serializing_if = "Option::is_none")]
    pub individual_identifiers: Option<Vec<crate::models::SupportedIdentifier>>,
    #[serde(rename = "organizationIdentifiers", skip_serializing_if = "Option::is_none")]
    pub organization_identifiers: Option<Vec<crate::models::SupportedIdentifier>>,
}

impl SupportedIdentifiersByCountry {
    pub fn new() -> SupportedIdentifiersByCountry {
        SupportedIdentifiersByCountry {
            iso2: None,
            name: None,
            flag: None,
            individual_identifiers: None,
            organization_identifiers: None,
        }
    }
}


