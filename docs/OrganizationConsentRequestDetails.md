# OrganizationConsentRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**receiver** | **String** | Name of request receiver organization. | 
**id** | **String** | Consent request id | 
**template_id** | Option<**String**> | Consent request template id | [optional]
**consent_id** | Option<**String**> | Consent id | [optional]
**title** | **String** | Consent request title. | 
**description** | **String** | Consent request description. | 
**purpose** | Option<**String**> | Consent request purpose. | [optional]
**status** | [**crate::models::DataConsentStatus**](DataConsentStatus.md) |  | 
**transaction_id** | Option<**String**> | Transaction id | [optional]
**created_at_utc** | **String** | Request creation datetime in UTC timezone | 
**expires_at_utc** | **String** | Request expiration datetime in UTC timezone | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


