/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// IssuedDocument : Issued Document Identifier.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssuedDocument {
    /// Document Id.
    #[serde(rename = "id")]
    pub id: String,
    /// Document Identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Document type name.
    #[serde(rename = "documentType")]
    pub document_type: String,
    /// User name.
    #[serde(rename = "issuedTo")]
    pub issued_to: String,
    /// Issued datetime in UTC timezone.
    #[serde(rename = "issuedAtUtc")]
    pub issued_at_utc: String,
}

impl IssuedDocument {
    /// Issued Document Identifier.
    pub fn new(id: String, identifier: String, document_type: String, issued_to: String, issued_at_utc: String) -> IssuedDocument {
        IssuedDocument {
            id,
            identifier,
            document_type,
            issued_to,
            issued_at_utc,
        }
    }
}


