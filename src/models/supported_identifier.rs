/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// SupportedIdentifier : Supported identifier details for a particular country.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SupportedIdentifier {
    /// Country ISO 2 code. Example: IN, US, etc.
    #[serde(rename = "iso2")]
    pub iso2: String,
    /// Country name. Example: India, United States of America, etc.
    #[serde(rename = "name")]
    pub name: String,
    /// List of supported identifiers for an individual.
    #[serde(rename = "individualIdentifiers")]
    pub individual_identifiers: Vec<crate::models::Identifier>,
    /// List of supported identifiers for an organization.
    #[serde(rename = "organizationIdentifiers")]
    pub organization_identifiers: Vec<crate::models::Identifier>,
}

impl SupportedIdentifier {
    /// Supported identifier details for a particular country.
    pub fn new(iso2: String, name: String, individual_identifiers: Vec<crate::models::Identifier>, organization_identifiers: Vec<crate::models::Identifier>) -> SupportedIdentifier {
        SupportedIdentifier {
            iso2,
            name,
            individual_identifiers,
            organization_identifiers,
        }
    }
}


