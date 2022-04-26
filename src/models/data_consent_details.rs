/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// DataConsentDetails : Data Consent details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataConsentDetails {
    /// Data consent id.
    #[serde(rename = "id")]
    pub id: String,
    /// Consent request id.
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// Consent template id.
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// Consent title.
    #[serde(rename = "title")]
    pub title: String,
    /// Consent description.
    #[serde(rename = "description")]
    pub description: String,
    /// Consent purpose.
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(rename = "status")]
    pub status: crate::models::DataConsentStatus,
    /// Transaction id.
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Consent requested datetime in UTC timezone.
    #[serde(rename = "requestedAtUtc")]
    pub requested_at_utc: String,
    /// Consent approval datetime in UTC timezone.
    #[serde(rename = "approvedAtUtc")]
    pub approved_at_utc: String,
    /// Data access expiration datetime in UTC timezone.
    #[serde(rename = "dataAccessExpiresAtUtc")]
    pub data_access_expires_at_utc: String,
    /// Consent revocation datetime in UTC timezone.
    #[serde(rename = "revokedAtUtc", skip_serializing_if = "Option::is_none")]
    pub revoked_at_utc: Option<String>,
}

impl DataConsentDetails {
    /// Data Consent details.
    pub fn new(id: String, request_id: String, title: String, description: String, status: crate::models::DataConsentStatus, requested_at_utc: String, approved_at_utc: String, data_access_expires_at_utc: String) -> DataConsentDetails {
        DataConsentDetails {
            id,
            request_id,
            template_id: None,
            title,
            description,
            purpose: None,
            status,
            transaction_id: None,
            requested_at_utc,
            approved_at_utc,
            data_access_expires_at_utc,
            revoked_at_utc: None,
        }
    }
}


