# DataConsentDetailsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::DataConsentStatus**](DataConsentStatus.md)> |  | [optional]
**approved_at_utc** | Option<**String**> |  | [optional]
**rejected_at_utc** | Option<**String**> |  | [optional]
**expires_at_utc** | Option<**String**> |  | [optional]
**requested_at_utc** | Option<**String**> |  | [optional]
**requester** | Option<[**crate::models::DataConsentRequesterDto**](DataConsentRequesterDto.md)> |  | [optional]
**consent_details** | Option<[**crate::models::GetConsentTemplateDetailsDto**](GetConsentTemplateDetailsDto.md)> |  | [optional]
**identifiers** | Option<[**Vec<crate::models::DataConsentIdentifier>**](DataConsentIdentifier.md)> |  | [optional]
**approved_documents** | Option<[**Vec<crate::models::DataConsentRequestedDocument>**](DataConsentRequestedDocument.md)> |  | [optional]
**approved_financials** | Option<[**Vec<crate::models::DataConsentRequestedFinancialAccount>**](DataConsentRequestedFinancialAccount.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


