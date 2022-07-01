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
pub enum DocumentSubCategoryType {
    #[serde(rename = "Identity")]
    Identity,
    #[serde(rename = "Property")]
    Property,
    #[serde(rename = "Vehicle")]
    Vehicle,
    #[serde(rename = "AdmitAndIdCards")]
    AdmitAndIdCards,
    #[serde(rename = "Marksheets")]
    Marksheets,
    #[serde(rename = "Transcripts")]
    Transcripts,
    #[serde(rename = "Certificates")]
    Certificates,
    #[serde(rename = "Prescriptions")]
    Prescriptions,
    #[serde(rename = "MedicalReports")]
    MedicalReports,
    #[serde(rename = "DischargeSummary")]
    DischargeSummary,
    #[serde(rename = "AccountAndCertificates")]
    AccountAndCertificates,
    #[serde(rename = "BillsAndInvoices")]
    BillsAndInvoices,
    #[serde(rename = "Insurance")]
    Insurance,
    #[serde(rename = "ReceiptsAndSubscriptions")]
    ReceiptsAndSubscriptions,
    #[serde(rename = "Taxes")]
    Taxes,
    #[serde(rename = "Corporates")]
    Corporates,
    #[serde(rename = "Competitions")]
    Competitions,
    #[serde(rename = "IntellectualProperties")]
    IntellectualProperties,
    #[serde(rename = "Others")]
    Others,
    #[serde(rename = "ApplicationsAndCertificates")]
    ApplicationsAndCertificates,
    #[serde(rename = "Licenses")]
    Licenses,
    #[serde(rename = "OrganizationIds")]
    OrganizationIds,
    #[serde(rename = "Permits")]
    Permits,
    #[serde(rename = "Legal")]
    Legal,

}

impl ToString for DocumentSubCategoryType {
    fn to_string(&self) -> String {
        match self {
            Self::Identity => String::from("Identity"),
            Self::Property => String::from("Property"),
            Self::Vehicle => String::from("Vehicle"),
            Self::AdmitAndIdCards => String::from("AdmitAndIdCards"),
            Self::Marksheets => String::from("Marksheets"),
            Self::Transcripts => String::from("Transcripts"),
            Self::Certificates => String::from("Certificates"),
            Self::Prescriptions => String::from("Prescriptions"),
            Self::MedicalReports => String::from("MedicalReports"),
            Self::DischargeSummary => String::from("DischargeSummary"),
            Self::AccountAndCertificates => String::from("AccountAndCertificates"),
            Self::BillsAndInvoices => String::from("BillsAndInvoices"),
            Self::Insurance => String::from("Insurance"),
            Self::ReceiptsAndSubscriptions => String::from("ReceiptsAndSubscriptions"),
            Self::Taxes => String::from("Taxes"),
            Self::Corporates => String::from("Corporates"),
            Self::Competitions => String::from("Competitions"),
            Self::IntellectualProperties => String::from("IntellectualProperties"),
            Self::Others => String::from("Others"),
            Self::ApplicationsAndCertificates => String::from("ApplicationsAndCertificates"),
            Self::Licenses => String::from("Licenses"),
            Self::OrganizationIds => String::from("OrganizationIds"),
            Self::Permits => String::from("Permits"),
            Self::Legal => String::from("Legal"),
        }
    }
}

impl Default for DocumentSubCategoryType {
    fn default() -> DocumentSubCategoryType {
        Self::Identity
    }
}




