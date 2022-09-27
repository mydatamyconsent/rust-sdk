/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// DocumentField : DocumentField : Document field of consent request template.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentField {
    /// Field title.
    #[serde(rename = "fieldTitle")]
    pub field_title: String,
    /// Field slug.
    #[serde(rename = "fieldSlug")]
    pub field_slug: String,
    /// Field DRNs.
    #[serde(rename = "drns")]
    pub drns: Vec<String>,
}

impl DocumentField {
    /// DocumentField : Document field of consent request template.
    pub fn new(field_title: String, field_slug: String, drns: Vec<String>) -> DocumentField {
        DocumentField {
            field_title,
            field_slug,
            drns,
        }
    }
}


