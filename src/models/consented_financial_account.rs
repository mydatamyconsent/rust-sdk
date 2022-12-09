/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// ConsentedFinancialAccount : ConsentedFinancialAccount : Consented financial account details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsentedFinancialAccount {
    /// Financial account id.
    #[serde(rename = "id")]
    pub id: String,
    /// Financial account name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "category")]
    pub category: Box<crate::models::FinancialAccountCategoryType>,
    #[serde(rename = "subCategory")]
    pub sub_category: Box<crate::models::FinancialAccountSubCategoryType>,
    /// Financial account identifier.
    #[serde(rename = "identifier")]
    pub identifier: String,
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
    pub transaction_period: Option<Box<crate::models::ConsentedFinancialAccountTransactionPeriod>>,
    /// Financial account issuer id.
    #[serde(rename = "issuerId")]
    pub issuer_id: String,
    /// Financial account issuer name.
    #[serde(rename = "issuerName")]
    pub issuer_name: String,
}

impl ConsentedFinancialAccount {
    /// ConsentedFinancialAccount : Consented financial account details.
    pub fn new(id: String, name: String, category: crate::models::FinancialAccountCategoryType, sub_category: crate::models::FinancialAccountSubCategoryType, identifier: String, field_title: String, field_slug: String, requested_details: Vec<crate::models::FinancialAccountDetailsRequired>, issuer_id: String, issuer_name: String) -> ConsentedFinancialAccount {
        ConsentedFinancialAccount {
            id,
            name,
            category: Box::new(category),
            sub_category: Box::new(sub_category),
            identifier,
            field_title,
            field_slug,
            requested_details,
            transaction_period: None,
            issuer_id,
            issuer_name,
        }
    }
}

