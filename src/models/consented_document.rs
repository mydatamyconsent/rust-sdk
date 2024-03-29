/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// ConsentedDocument : ConsentedDocument : Consented document details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsentedDocument {
    /// Document id.
    #[serde(rename = "id")]
    pub id: String,
    /// Document name.
    #[serde(rename = "name")]
    pub name: String,
    /// Document category.
    #[serde(rename = "category")]
    pub category: String,
    /// Document identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Document field title.
    #[serde(rename = "fieldTitle")]
    pub field_title: String,
    /// Document field slug.
    #[serde(rename = "fieldSlug")]
    pub field_slug: String,
    /// Document issued at datetime in UTC timezone.
    #[serde(rename = "issuedAtUtc")]
    pub issued_at_utc: String,
    /// Document expires at datetime in UTC timezone.
    #[serde(rename = "expiresAtUtc", skip_serializing_if = "Option::is_none")]
    pub expires_at_utc: Option<String>,
    #[serde(rename = "issuer")]
    pub issuer: Box<crate::models::ConsentDocumentIssuer>,
    /// Digital signatures.
    #[serde(rename = "digitalSignatures")]
    pub digital_signatures: Vec<crate::models::DocumentDigitalSignature>,
}

impl ConsentedDocument {
    /// ConsentedDocument : Consented document details.
    pub fn new(id: String, name: String, category: String, identifier: String, field_title: String, field_slug: String, issued_at_utc: String, issuer: crate::models::ConsentDocumentIssuer, digital_signatures: Vec<crate::models::DocumentDigitalSignature>) -> ConsentedDocument {
        ConsentedDocument {
            id,
            name,
            category,
            identifier,
            field_title,
            field_slug,
            issued_at_utc,
            expires_at_utc: None,
            issuer: Box::new(issuer),
            digital_signatures,
        }
    }
}


