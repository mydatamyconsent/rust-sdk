/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// Identifier : Identifier details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Identifier {
    /// Identifier key. EMAIL, MOBILE_NUMBER, etc.
    #[serde(rename = "key")]
    pub key: String,
    /// Identifier name. Email, Mobile Number, etc.
    #[serde(rename = "name")]
    pub name: String,
    /// Identifier description. User's email, User's mobile number, etc.
    #[serde(rename = "description")]
    pub description: String,
    /// Example value. example@email.com, +919090909090, etc.
    #[serde(rename = "example_value")]
    pub example_value: String,
}

impl Identifier {
    /// Identifier details.
    pub fn new(key: String, name: String, description: String, example_value: String) -> Identifier {
        Identifier {
            key,
            name,
            description,
            example_value,
        }
    }
}


