/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// ConsentedFinancialAccountField : ConsentedFinancialAccountField : Consented financial account field details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsentedFinancialAccountField {
    /// Financial account field title.
    #[serde(rename = "fieldTitle")]
    pub field_title: String,
    /// Financial account field slug.
    #[serde(rename = "fieldSlug")]
    pub field_slug: String,
    /// Requested financial account details.
    #[serde(rename = "requestedDetails")]
    pub requested_details: Vec<crate::models::FinancialAccountDetailsRequired>,
    #[serde(rename = "transactionPeriod", skip_serializing_if = "Option::is_none")]
    pub transaction_period: Option<Box<crate::models::ConsentedFinancialAccountFieldTransactionPeriod>>,
    /// Consented financial accounts.
    #[serde(rename = "consentedAccounts")]
    pub consented_accounts: Vec<crate::models::ConsentedFinancialAccount>,
}

impl ConsentedFinancialAccountField {
    /// ConsentedFinancialAccountField : Consented financial account field details.
    pub fn new(field_title: String, field_slug: String, requested_details: Vec<crate::models::FinancialAccountDetailsRequired>, consented_accounts: Vec<crate::models::ConsentedFinancialAccount>) -> ConsentedFinancialAccountField {
        ConsentedFinancialAccountField {
            field_title,
            field_slug,
            requested_details,
            transaction_period: None,
            consented_accounts,
        }
    }
}


