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
pub enum ErrorType {
    #[serde(rename = "InvalidAccessToken")]
    InvalidAccessToken,
    #[serde(rename = "InvalidRefreshToken")]
    InvalidRefreshToken,
    #[serde(rename = "InsufficientPermission")]
    InsufficientPermission,
    #[serde(rename = "InternalServerError")]
    InternalServerError,
    #[serde(rename = "BadRequest")]
    BadRequest,
    #[serde(rename = "NotFound")]
    NotFound,
    #[serde(rename = "InvalidOrganization")]
    InvalidOrganization,
    #[serde(rename = "InvalidFileUploadType")]
    InvalidFileUploadType,

}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::InvalidAccessToken => String::from("InvalidAccessToken"),
            Self::InvalidRefreshToken => String::from("InvalidRefreshToken"),
            Self::InsufficientPermission => String::from("InsufficientPermission"),
            Self::InternalServerError => String::from("InternalServerError"),
            Self::BadRequest => String::from("BadRequest"),
            Self::NotFound => String::from("NotFound"),
            Self::InvalidOrganization => String::from("InvalidOrganization"),
            Self::InvalidFileUploadType => String::from("InvalidFileUploadType"),
        }
    }
}

impl Default for ErrorType {
    fn default() -> ErrorType {
        Self::InvalidAccessToken
    }
}




