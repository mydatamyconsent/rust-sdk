/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// ConsentDetails : ConsentDetails : Consent details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsentDetails {
    /// Consent id.
    #[serde(rename = "id")]
    pub id: String,
    /// Consent request id.
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// Consent request template id.
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
    /// Consent approval datetime in UTC timezone.
    #[serde(rename = "approvedAtUtc")]
    pub approved_at_utc: String,
    /// Data access expiration datetime in UTC timezone.
    #[serde(rename = "dataAccessExpiresAtUtc")]
    pub data_access_expires_at_utc: String,
    /// Consent revocation datetime in UTC timezone.
    #[serde(rename = "revokedAtUtc", skip_serializing_if = "Option::is_none")]
    pub revoked_at_utc: Option<String>,
    /// List of supported collectible types.
    #[serde(rename = "collectables")]
    pub collectables: Vec<crate::models::CollectibleTypes>,
    /// Consented identity details.
    #[serde(rename = "identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<crate::models::ConsentedIdentifier>>,
    /// List of consented documents.
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<crate::models::ConsentedDocument>>,
    /// List of consented medical records.
    #[serde(rename = "medicalRecords", skip_serializing_if = "Option::is_none")]
    pub medical_records: Option<Vec<crate::models::ConsentedMedicalRecord>>,
    /// List of consented financial accounts.
    #[serde(rename = "financialAccounts", skip_serializing_if = "Option::is_none")]
    pub financial_accounts: Option<Vec<crate::models::ConsentedFinancialAccountField>>,
}

impl ConsentDetails {
    /// ConsentDetails : Consent details.
    pub fn new(id: String, request_id: String, title: String, description: String, status: crate::models::DataConsentStatus, approved_at_utc: String, data_access_expires_at_utc: String, collectables: Vec<crate::models::CollectibleTypes>) -> ConsentDetails {
        ConsentDetails {
            id,
            request_id,
            template_id: None,
            title,
            description,
            purpose: None,
            status,
            transaction_id: None,
            approved_at_utc,
            data_access_expires_at_utc,
            revoked_at_utc: None,
            collectables,
            identifiers: None,
            documents: None,
            medical_records: None,
            financial_accounts: None,
        }
    }
}


