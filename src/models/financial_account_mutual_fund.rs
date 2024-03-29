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
pub struct FinancialAccountMutualFund {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "investment_value")]
    pub investment_value: f64,
    #[serde(rename = "current_value")]
    pub current_value: f64,
    #[serde(rename = "currency_code")]
    pub currency_code: String,
    #[serde(rename = "amc", skip_serializing_if = "Option::is_none")]
    pub amc: Option<String>,
    #[serde(rename = "registrar", skip_serializing_if = "Option::is_none")]
    pub registrar: Option<String>,
    #[serde(rename = "fund_name")]
    pub fund_name: String,
    #[serde(rename = "isin")]
    pub isin: String,
    #[serde(rename = "folio_number")]
    pub folio_number: String,
    #[serde(rename = "scheme_code", skip_serializing_if = "Option::is_none")]
    pub scheme_code: Option<String>,
    #[serde(rename = "fund_type", skip_serializing_if = "Option::is_none")]
    pub fund_type: Option<String>,
    #[serde(rename = "fund_category", skip_serializing_if = "Option::is_none")]
    pub fund_category: Option<String>,
    #[serde(rename = "units")]
    pub units: f64,
    #[serde(rename = "lien_units", skip_serializing_if = "Option::is_none")]
    pub lien_units: Option<String>,
    #[serde(rename = "creation_date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "holder")]
    pub holder: Box<crate::models::Holder>,
    #[serde(rename = "transactions")]
    pub transactions: bool,
}

impl FinancialAccountMutualFund {
    pub fn new(r#type: String, id: String, name: String, investment_value: f64, current_value: f64, currency_code: String, fund_name: String, isin: String, folio_number: String, units: f64, holder: crate::models::Holder, transactions: bool) -> FinancialAccountMutualFund {
        FinancialAccountMutualFund {
            r#type,
            id,
            name,
            investment_value,
            current_value,
            currency_code,
            amc: None,
            registrar: None,
            fund_name,
            isin,
            folio_number,
            scheme_code: None,
            fund_type: None,
            fund_category: None,
            units,
            lien_units: None,
            creation_date: None,
            holder: Box::new(holder),
            transactions,
        }
    }
}


