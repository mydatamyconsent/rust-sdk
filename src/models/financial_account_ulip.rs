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
pub struct FinancialAccountUlip {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "amount")]
    pub amount: f64,
}

impl FinancialAccountUlip {
    pub fn new(r#type: String, id: String, name: String, identifier: String, amount: f64) -> FinancialAccountUlip {
        FinancialAccountUlip {
            r#type,
            id,
            name,
            identifier,
            amount,
        }
    }
}


