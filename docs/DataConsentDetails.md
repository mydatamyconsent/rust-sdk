# DataConsentDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Data consent id. | 
**request_id** | **String** | Consent request id. | 
**template_id** | Option<**String**> | Consent template id. | [optional]
**title** | **String** | Consent title. | 
**description** | **String** | Consent description. | 
**purpose** | Option<**String**> | Consent purpose. | [optional]
**status** | [**crate::models::DataConsentStatus**](DataConsentStatus.md) |  | 
**transaction_id** | Option<**String**> | Transaction id. | [optional]
**requested_at_utc** | **String** | Consent requested datetime in UTC timezone. | 
**approved_at_utc** | **String** | Consent approval datetime in UTC timezone. | 
**data_access_expires_at_utc** | **String** | Data access expiration datetime in UTC timezone. | 
**revoked_at_utc** | Option<**String**> | Consent revocation datetime in UTC timezone. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


