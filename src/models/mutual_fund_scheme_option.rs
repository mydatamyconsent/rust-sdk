/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MutualFundSchemeOption {
    #[serde(rename = "Reinvest")]
    Reinvest,
    #[serde(rename = "Payout")]
    Payout,
    #[serde(rename = "GrowthType")]
    GrowthType,

}

impl ToString for MutualFundSchemeOption {
    fn to_string(&self) -> String {
        match self {
            Self::Reinvest => String::from("Reinvest"),
            Self::Payout => String::from("Payout"),
            Self::GrowthType => String::from("GrowthType"),
        }
    }
}

impl Default for MutualFundSchemeOption {
    fn default() -> MutualFundSchemeOption {
        Self::Reinvest
    }
}



