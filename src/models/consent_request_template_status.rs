/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// ConsentRequestTemplateStatus : Consent request template status.

/// Consent request template status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConsentRequestTemplateStatus {
    #[serde(rename = "CreationPending")]
    CreationPending,
    #[serde(rename = "UnderReview")]
    UnderReview,
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "Rejected")]
    Rejected,

}

impl ToString for ConsentRequestTemplateStatus {
    fn to_string(&self) -> String {
        match self {
            Self::CreationPending => String::from("CreationPending"),
            Self::UnderReview => String::from("UnderReview"),
            Self::Active => String::from("Active"),
            Self::Inactive => String::from("Inactive"),
            Self::Rejected => String::from("Rejected"),
        }
    }
}

impl Default for ConsentRequestTemplateStatus {
    fn default() -> ConsentRequestTemplateStatus {
        Self::CreationPending
    }
}



