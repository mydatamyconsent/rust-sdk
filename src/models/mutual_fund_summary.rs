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
pub struct MutualFundSummary {
    #[serde(rename = "investment")]
    pub investment: Box<crate::models::MutualFundInvestment>,
    #[serde(rename = "investmentValue")]
    pub investment_value: f64,
    #[serde(rename = "currentValue")]
    pub current_value: f64,
}

impl MutualFundSummary {
    pub fn new(investment: crate::models::MutualFundInvestment, investment_value: f64, current_value: f64) -> MutualFundSummary {
        MutualFundSummary {
            investment: Box::new(investment),
            investment_value,
            current_value,
        }
    }
}


