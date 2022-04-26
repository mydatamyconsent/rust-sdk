# DataConsent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Data consent id. | 
**request_id** | **String** | Data consent request id. | 
**template_id** | Option<**String**> | Consent template id. | [optional]
**title** | **String** | Consent title. | 
**description** | **String** | Consent description. | 
**purpose** | Option<**String**> | Consent purpose. | [optional]
**status** | [**crate::models::DataConsentStatus**](DataConsentStatus.md) |  | 
**transaction_id** | Option<**String**> | Transaction id. | [optional]
**approved_at_utc** | **String** | Consent approval datetime in UTC timezone. | 
**data_access_expires_at_utc** | **String** | Data access expiration datetime in UTC timezone. | 
**revoked_at_utc** | Option<**String**> | Consent revocation datetime in UTC timezone. | [optional]
**collectables** | [**Vec<crate::models::CollectibleTypes>**](CollectibleTypes.md) | List of supported collectible types. | 
**identifiers** | Option<[**serde_json::Value**](.md)> | Consented identity details. | [optional]
**documents** | Option<[**Vec<crate::models::DataConsentDocument>**](DataConsentDocument.md)> | List of consented documents. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


