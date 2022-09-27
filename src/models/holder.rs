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
pub struct Holder {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "dob", skip_serializing_if = "Option::is_none")]
    pub dob: Option<String>,
    #[serde(rename = "mobile", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "nominee", skip_serializing_if = "Option::is_none")]
    pub nominee: Option<crate::models::HoldingNominee>,
    #[serde(rename = "dematId")]
    pub demat_id: String,
    #[serde(rename = "landline", skip_serializing_if = "Option::is_none")]
    pub landline: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "pan", skip_serializing_if = "Option::is_none")]
    pub pan: Option<String>,
    #[serde(rename = "ckycCompliance")]
    pub ckyc_compliance: bool,
}

impl Holder {
    pub fn new(name: String, demat_id: String, email: String, ckyc_compliance: bool) -> Holder {
        Holder {
            name,
            dob: None,
            mobile: None,
            nominee: None,
            demat_id,
            landline: None,
            address: None,
            email,
            pan: None,
            ckyc_compliance,
        }
    }
}


