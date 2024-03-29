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
pub struct FinancialAccountTransactionSipTransaction {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "currency_code")]
    pub currency_code: String,
    #[serde(rename = "txn_type")]
    pub txn_type: crate::models::SipTransactionType,
    #[serde(rename = "transacted_at_utc")]
    pub transacted_at_utc: String,
}

impl FinancialAccountTransactionSipTransaction {
    pub fn new(r#type: String, id: String, amount: f64, currency_code: String, txn_type: crate::models::SipTransactionType, transacted_at_utc: String) -> FinancialAccountTransactionSipTransaction {
        FinancialAccountTransactionSipTransaction {
            r#type,
            id,
            amount,
            currency_code,
            txn_type,
            transacted_at_utc,
        }
    }
}


