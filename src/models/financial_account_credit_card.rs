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
pub struct FinancialAccountCreditCard {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "balance")]
    pub balance: f64,
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::CreditCardProfile>,
    #[serde(rename = "summary")]
    pub summary: Box<crate::models::CreditCardSummary>,
    #[serde(rename = "masked_account_number")]
    pub masked_account_number: String,
    #[serde(rename = "linked_account_ref")]
    pub linked_account_ref: String,
    #[serde(rename = "version")]
    pub version: f32,
}

impl FinancialAccountCreditCard {
    pub fn new(r#type: String, id: String, name: String, identifier: String, balance: f64, profile: crate::models::CreditCardProfile, summary: crate::models::CreditCardSummary, masked_account_number: String, linked_account_ref: String, version: f32) -> FinancialAccountCreditCard {
        FinancialAccountCreditCard {
            r#type,
            id,
            name,
            identifier,
            balance,
            profile: Box::new(profile),
            summary: Box::new(summary),
            masked_account_number,
            linked_account_ref,
            version,
        }
    }
}


