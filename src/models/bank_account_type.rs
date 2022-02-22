/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankAccountType {
    #[serde(rename = "SavingsAccount")]
    SavingsAccount,
    #[serde(rename = "CurrentAccount")]
    CurrentAccount,

}

impl ToString for BankAccountType {
    fn to_string(&self) -> String {
        match self {
            Self::SavingsAccount => String::from("SavingsAccount"),
            Self::CurrentAccount => String::from("CurrentAccount"),
        }
    }
}

impl Default for BankAccountType {
    fn default() -> BankAccountType {
        Self::SavingsAccount
    }
}




