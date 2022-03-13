/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// IndividualDataConsentDocument : Individual data consent document details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IndividualDataConsentDocument {
    /// Name of consent approver individual.
    #[serde(rename = "approver")]
    pub approver: String,
    /// Document id.
    #[serde(rename = "id")]
    pub id: String,
    /// Data consent id.
    #[serde(rename = "consentId")]
    pub consent_id: String,
    /// Document name.
    #[serde(rename = "name")]
    pub name: String,
    /// Document identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
}

impl IndividualDataConsentDocument {
    /// Individual data consent document details.
    pub fn new(approver: String, id: String, consent_id: String, name: String, identifier: String) -> IndividualDataConsentDocument {
        IndividualDataConsentDocument {
            approver,
            id,
            consent_id,
            name,
            identifier,
        }
    }
}


