/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// DocumentIssueRequest : Document Issue Request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentIssueRequest {
    /// Document type id.
    #[serde(rename = "documentTypeId")]
    pub document_type_id: String,
    /// Document identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Document description.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "receiver")]
    pub receiver: Box<crate::models::DocumentReceiver>,
    /// Datetime of issue in UTC timezone.
    #[serde(rename = "issuedAtUtc")]
    pub issued_at_utc: String,
    /// Valid from datetime in UTC timezone.
    #[serde(rename = "validFromUtc")]
    pub valid_from_utc: String,
    /// Datetime of expiry in UTC timezone.
    #[serde(rename = "expiresAtUtc", skip_serializing_if = "Option::is_none")]
    pub expires_at_utc: Option<String>,
    #[serde(rename = "paymentRequest", skip_serializing_if = "Option::is_none")]
    pub payment_request: Option<Box<crate::models::PaymentRequest>>,
    /// Metadata.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
}

impl DocumentIssueRequest {
    /// Document Issue Request.
    pub fn new(document_type_id: String, identifier: String, description: String, receiver: crate::models::DocumentReceiver, issued_at_utc: String, valid_from_utc: String) -> DocumentIssueRequest {
        DocumentIssueRequest {
            document_type_id,
            identifier,
            description,
            receiver: Box::new(receiver),
            issued_at_utc,
            valid_from_utc,
            expires_at_utc: None,
            payment_request: None,
            metadata: None,
        }
    }
}


