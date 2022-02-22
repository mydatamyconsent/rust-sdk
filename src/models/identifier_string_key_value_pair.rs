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
pub struct IdentifierStringKeyValuePair {
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<crate::models::Identifier>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl IdentifierStringKeyValuePair {
    pub fn new() -> IdentifierStringKeyValuePair {
        IdentifierStringKeyValuePair {
            key: None,
            value: None,
        }
    }
}


