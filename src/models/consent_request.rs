/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// ConsentRequest : ConsentRequest :Consent request details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsentRequest {
    /// Consent request id.
    #[serde(rename = "id")]
    pub id: String,
    /// Consent request template id.
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// Consent id.
    #[serde(rename = "consentId", skip_serializing_if = "Option::is_none")]
    pub consent_id: Option<String>,
    /// Consent title.
    #[serde(rename = "title")]
    pub title: String,
    /// Consent description.
    #[serde(rename = "description")]
    pub description: String,
    /// Consent purpose.
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(rename = "dataLife", skip_serializing_if = "Option::is_none")]
    pub data_life: Option<Box<crate::models::Life>>,
    /// List of supported collectables.
    #[serde(rename = "collectables")]
    pub collectables: Vec<crate::models::CollectibleTypes>,
    #[serde(rename = "receiver")]
    pub receiver: Box<crate::models::ConsentRequestReceiver>,
    #[serde(rename = "status")]
    pub status: crate::models::DataConsentStatus,
    /// Request creation datetime in UTC timezone.
    #[serde(rename = "createdAtUtc")]
    pub created_at_utc: String,
    /// Request expiration datetime in UTC timezone.
    #[serde(rename = "expiresAtUtc")]
    pub expires_at_utc: String,
    /// Request approval datetime in UTC timezone.
    #[serde(rename = "approvedAtUtc", skip_serializing_if = "Option::is_none")]
    pub approved_at_utc: Option<String>,
    /// Data access expiration datetime in UTC timezone.
    #[serde(rename = "dataAccessExpiresAtUtc", skip_serializing_if = "Option::is_none")]
    pub data_access_expires_at_utc: Option<String>,
    /// Request rejection datetime in UTC timezone.
    #[serde(rename = "rejectedAtUtc", skip_serializing_if = "Option::is_none")]
    pub rejected_at_utc: Option<String>,
    /// Request revocation datetime in UTC timezone.
    #[serde(rename = "revokedAtUtc", skip_serializing_if = "Option::is_none")]
    pub revoked_at_utc: Option<String>,
}

impl ConsentRequest {
    /// ConsentRequest :Consent request details.
    pub fn new(id: String, title: String, description: String, collectables: Vec<crate::models::CollectibleTypes>, receiver: crate::models::ConsentRequestReceiver, status: crate::models::DataConsentStatus, created_at_utc: String, expires_at_utc: String) -> ConsentRequest {
        ConsentRequest {
            id,
            template_id: None,
            consent_id: None,
            title,
            description,
            purpose: None,
            data_life: None,
            collectables,
            receiver: Box::new(receiver),
            status,
            created_at_utc,
            expires_at_utc,
            approved_at_utc: None,
            data_access_expires_at_utc: None,
            rejected_at_utc: None,
            revoked_at_utc: None,
        }
    }
}


