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
pub struct DocumentIssueRequestPaymentRequest {
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "items")]
    pub items: Vec<crate::models::PaymentOrderItem>,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "paymentUrl", skip_serializing_if = "Option::is_none")]
    pub payment_url: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "dueByUtc")]
    pub due_by_utc: String,
}

impl DocumentIssueRequestPaymentRequest {
    pub fn new(identifier: String, items: Vec<crate::models::PaymentOrderItem>, currency_code: String, description: String, due_by_utc: String) -> DocumentIssueRequestPaymentRequest {
        DocumentIssueRequestPaymentRequest {
            identifier,
            items,
            currency_code,
            payment_url: None,
            description,
            due_by_utc,
        }
    }
}


