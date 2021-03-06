/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// DataProcessingAgreement : Data processing agreement details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataProcessingAgreement {
    /// Agreement id.
    #[serde(rename = "id")]
    pub id: String,
    /// Agreement version.
    #[serde(rename = "version")]
    pub version: String,
    /// Agreement body content.
    #[serde(rename = "body")]
    pub body: String,
    /// Agreement attachment file URL.
    #[serde(rename = "attachmentUrl")]
    pub attachment_url: String,
}

impl DataProcessingAgreement {
    /// Data processing agreement details.
    pub fn new(id: String, version: String, body: String, attachment_url: String) -> DataProcessingAgreement {
        DataProcessingAgreement {
            id,
            version,
            body,
            attachment_url,
        }
    }
}


