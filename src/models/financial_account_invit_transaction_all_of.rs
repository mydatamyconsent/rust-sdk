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
pub struct FinancialAccountInvitTransactionAllOf {
    #[serde(rename = "type")]
    pub r#type: String,
}

impl FinancialAccountInvitTransactionAllOf {
    pub fn new(r#type: String) -> FinancialAccountInvitTransactionAllOf {
        FinancialAccountInvitTransactionAllOf {
            r#type,
        }
    }
}


