/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PushUriRequest {
    #[serde(rename = "uriDetails")]
    pub uri_details: Box<crate::models::UriDetails>,
    #[serde(rename = "ns2", skip_serializing_if = "Option::is_none")]
    pub ns2: Option<String>,
    #[serde(rename = "ver", skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename = "txn", skip_serializing_if = "Option::is_none")]
    pub txn: Option<String>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "keyHash", skip_serializing_if = "Option::is_none")]
    pub key_hash: Option<String>,
}

impl PushUriRequest {
    pub fn new(uri_details: crate::models::UriDetails) -> PushUriRequest {
        PushUriRequest {
            uri_details: Box::new(uri_details),
            ns2: None,
            ver: None,
            ts: None,
            txn: None,
            org_id: None,
            key_hash: None,
        }
    }
}


