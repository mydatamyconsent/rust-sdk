/*
 * My Data My Consent - Developer API
 *
 * Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@mydatamyconsent.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateOrganizationDataConsentRequest : Organization Data Consent Request.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateOrganizationDataConsentRequest {
    #[serde(rename = "consentTemplateId", skip_serializing_if = "Option::is_none")]
    pub consent_template_id: Option<String>,
    #[serde(rename = "receiver")]
    pub receiver: Box<crate::models::Receiver>,
}

impl CreateOrganizationDataConsentRequest {
    /// Organization Data Consent Request.
    pub fn new(receiver: crate::models::Receiver) -> CreateOrganizationDataConsentRequest {
        CreateOrganizationDataConsentRequest {
            consent_template_id: None,
            receiver: Box::new(receiver),
        }
    }
}


