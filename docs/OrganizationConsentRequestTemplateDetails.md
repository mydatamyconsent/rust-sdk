# OrganizationConsentRequestTemplateDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Consent request template id. | 
**title** | **String** | Consent request template title. | 
**description** | **String** | Consent request template description. | 
**purpose** | Option<**String**> | Consent request template purpose. | [optional]
**short_id** | **String** | Consent request template short Id. | 
**status** | [**crate::models::ConsentRequestTemplateStatus**](ConsentRequestTemplateStatus.md) |  | 
**data_life** | Option<[**crate::models::IndividualConsentRequestTemplateDetailsDataLife**](IndividualConsentRequestTemplateDetails_dataLife.md)> |  | [optional]
**request_life** | Option<[**crate::models::IndividualConsentRequestTemplateDetailsRequestLife**](IndividualConsentRequestTemplateDetails_requestLife.md)> |  | [optional]
**data_frequency** | Option<[**crate::models::IndividualConsentRequestTemplateDetailsDataFrequency**](IndividualConsentRequestTemplateDetails_dataFrequency.md)> |  | [optional]
**identifiers** | Option<[**Vec<crate::models::IdentityField>**](IdentityField.md)> | Consent request template identity fields. | [optional]
**documents** | Option<[**Vec<crate::models::DocumentField>**](DocumentField.md)> | Consent request template document fields. | [optional]
**financial_accounts** | Option<[**Vec<crate::models::FinancialAccountField>**](FinancialAccountField.md)> | Consent request template financial account fields. | [optional]
**created_by** | **String** | Consent request template created by user. | 
**created_at_utc** | **String** | Consent request template created datetime in UTC timezone. | 
**approved_at_utc** | Option<**String**> | Consent request template approval datetime in UTC timezone. | [optional]
**published_at_utc** | Option<**String**> | Consent request template published datetime in UTC timezone. | [optional]
**rejected_at_utc** | Option<**String**> | Consent request template rejection datetime in UTC timezone. | [optional]
**rejection_reason** | Option<**String**> | Consent request template rejection reason. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


