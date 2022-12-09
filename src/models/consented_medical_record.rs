/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// ConsentedMedicalRecord : ConsentedMedicalRecord : Consented medical record details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsentedMedicalRecord {
    /// Health id.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Health field title.
    #[serde(rename = "fieldTitle")]
    pub field_title: String,
    /// Health field slug.
    #[serde(rename = "fieldSlug")]
    pub field_slug: String,
    /// Issuer id.
    #[serde(rename = "issuerId")]
    pub issuer_id: String,
    /// Issuer name.
    #[serde(rename = "issuerName")]
    pub issuer_name: String,
    /// health category type.
    #[serde(rename = "category")]
    pub category: String,
    /// To Date
    #[serde(rename = "toDate", skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// From Date
    #[serde(rename = "fromDate", skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
}

impl ConsentedMedicalRecord {
    /// ConsentedMedicalRecord : Consented medical record details.
    pub fn new(field_title: String, field_slug: String, issuer_id: String, issuer_name: String, category: String) -> ConsentedMedicalRecord {
        ConsentedMedicalRecord {
            id: None,
            field_title,
            field_slug,
            issuer_id,
            issuer_name,
            category,
            to_date: None,
            from_date: None,
        }
    }
}

