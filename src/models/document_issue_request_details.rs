/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// DocumentIssueRequestDetails : Document issue request details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentIssueRequestDetails {
    /// Document issue request Id.
    #[serde(rename = "id")]
    pub id: String,
    /// Document type Id.
    #[serde(rename = "documentTypeId")]
    pub document_type_id: String,
    /// Document type name.
    #[serde(rename = "typeName")]
    pub type_name: String,
    /// Document identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "status")]
    pub status: crate::models::DocumentIssueRequestStatus,
    /// Document description.
    #[serde(rename = "description")]
    pub description: String,
    /// Document receiver details.
    #[serde(rename = "receiver")]
    pub receiver: Option<serde_json::Value>,
    /// Datetime of issue in UTC timezone.
    #[serde(rename = "issuedAtUtc")]
    pub issued_at_utc: String,
    /// Valid from datetime in UTC timezone.
    #[serde(rename = "validFromUtc")]
    pub valid_from_utc: String,
    /// Datetime of expiry in UTC timezone.
    #[serde(rename = "expiresAtUtc", skip_serializing_if = "Option::is_none")]
    pub expires_at_utc: Option<String>,
    /// Metadata.
    #[serde(rename = "metaData", skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<serde_json::Value>,
    /// Creation datetime of issue request in UTC timezone.
    #[serde(rename = "createdAtUtc")]
    pub created_at_utc: String,
}

impl DocumentIssueRequestDetails {
    /// Document issue request details.
    pub fn new(id: String, document_type_id: String, type_name: String, identifier: String, status: crate::models::DocumentIssueRequestStatus, description: String, receiver: Option<serde_json::Value>, issued_at_utc: String, valid_from_utc: String, created_at_utc: String) -> DocumentIssueRequestDetails {
        DocumentIssueRequestDetails {
            id,
            document_type_id,
            type_name,
            identifier,
            status,
            description,
            receiver,
            issued_at_utc,
            valid_from_utc,
            expires_at_utc: None,
            meta_data: None,
            created_at_utc,
        }
    }
}


