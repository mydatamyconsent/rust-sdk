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
pub enum TermDepositTransactionMode {
    #[serde(rename = "Cash")]
    Cash,
    #[serde(rename = "Atm")]
    Atm,
    #[serde(rename = "CardPayment")]
    CardPayment,
    #[serde(rename = "Upi")]
    Upi,
    #[serde(rename = "Ft")]
    Ft,
    #[serde(rename = "Others")]
    Others,

}

impl ToString for TermDepositTransactionMode {
    fn to_string(&self) -> String {
        match self {
            Self::Cash => String::from("Cash"),
            Self::Atm => String::from("Atm"),
            Self::CardPayment => String::from("CardPayment"),
            Self::Upi => String::from("Upi"),
            Self::Ft => String::from("Ft"),
            Self::Others => String::from("Others"),
        }
    }
}

impl Default for TermDepositTransactionMode {
    fn default() -> TermDepositTransactionMode {
        Self::Cash
    }
}




