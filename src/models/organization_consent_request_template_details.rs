/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationConsentRequestTemplateDetails : OrganizationConsentRequestTemplateDetails : Organization consent request template details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OrganizationConsentRequestTemplateDetails {
    /// Consent request template id.
    #[serde(rename = "id")]
    pub id: String,
    /// Consent request template title.
    #[serde(rename = "title")]
    pub title: String,
    /// Consent request template description.
    #[serde(rename = "description")]
    pub description: String,
    /// Consent request template purpose.
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Consent request template short Id.
    #[serde(rename = "shortId")]
    pub short_id: String,
    #[serde(rename = "status")]
    pub status: Box<crate::models::ConsentRequestTemplateStatus>,
    #[serde(rename = "dataLife", skip_serializing_if = "Option::is_none")]
    pub data_life: Option<Box<crate::models::IndividualConsentRequestTemplateDetailsDataLife>>,
    #[serde(rename = "requestLife", skip_serializing_if = "Option::is_none")]
    pub request_life: Option<Box<crate::models::IndividualConsentRequestTemplateDetailsRequestLife>>,
    #[serde(rename = "dataFrequency", skip_serializing_if = "Option::is_none")]
    pub data_frequency: Option<Box<crate::models::IndividualConsentRequestTemplateDetailsDataFrequency>>,
    /// Consent request template identity fields.
    #[serde(rename = "identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<crate::models::IdentityField>>,
    /// Consent request template document fields.
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<crate::models::DocumentField>>,
    /// Consent request template financial account fields.
    #[serde(rename = "financialAccounts", skip_serializing_if = "Option::is_none")]
    pub financial_accounts: Option<Vec<crate::models::FinancialAccountField>>,
    /// Consent request template created by user.
    #[serde(rename = "createdBy")]
    pub created_by: String,
    /// Consent request template created datetime in UTC timezone.
    #[serde(rename = "createdAtUtc")]
    pub created_at_utc: String,
    /// Consent request template approval datetime in UTC timezone.
    #[serde(rename = "approvedAtUtc", skip_serializing_if = "Option::is_none")]
    pub approved_at_utc: Option<String>,
    /// Consent request template published datetime in UTC timezone.
    #[serde(rename = "publishedAtUtc", skip_serializing_if = "Option::is_none")]
    pub published_at_utc: Option<String>,
    /// Consent request template rejection datetime in UTC timezone.
    #[serde(rename = "rejectedAtUtc", skip_serializing_if = "Option::is_none")]
    pub rejected_at_utc: Option<String>,
    /// Consent request template rejection reason.
    #[serde(rename = "rejectionReason", skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
}

impl OrganizationConsentRequestTemplateDetails {
    /// OrganizationConsentRequestTemplateDetails : Organization consent request template details.
    pub fn new(id: String, title: String, description: String, short_id: String, status: crate::models::ConsentRequestTemplateStatus, created_by: String, created_at_utc: String) -> OrganizationConsentRequestTemplateDetails {
        OrganizationConsentRequestTemplateDetails {
            id,
            title,
            description,
            purpose: None,
            short_id,
            status: Box::new(status),
            data_life: None,
            request_life: None,
            data_frequency: None,
            identifiers: None,
            documents: None,
            financial_accounts: None,
            created_by,
            created_at_utc,
            approved_at_utc: None,
            published_at_utc: None,
            rejected_at_utc: None,
            rejection_reason: None,
        }
    }
}


