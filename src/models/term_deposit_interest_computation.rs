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
pub enum TermDepositInterestComputation {
    #[serde(rename = "Simple")]
    Simple,
    #[serde(rename = "Compound")]
    Compound,

}

impl ToString for TermDepositInterestComputation {
    fn to_string(&self) -> String {
        match self {
            Self::Simple => String::from("Simple"),
            Self::Compound => String::from("Compound"),
        }
    }
}

impl Default for TermDepositInterestComputation {
    fn default() -> TermDepositInterestComputation {
        Self::Simple
    }
}




