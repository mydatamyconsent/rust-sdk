/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UriDetails {
    #[serde(rename = "aadhaar", skip_serializing_if = "Option::is_none")]
    pub aadhaar: Option<String>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "docType", skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    #[serde(rename = "docName", skip_serializing_if = "Option::is_none")]
    pub doc_name: Option<String>,
    #[serde(rename = "docId", skip_serializing_if = "Option::is_none")]
    pub doc_id: Option<String>,
    #[serde(rename = "issuedOn", skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,
    #[serde(rename = "validFrom", skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo", skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl UriDetails {
    pub fn new() -> UriDetails {
        UriDetails {
            aadhaar: None,
            uri: None,
            doc_type: None,
            doc_name: None,
            doc_id: None,
            issued_on: None,
            valid_from: None,
            valid_to: None,
            timestamp: None,
            action: None,
        }
    }
}


