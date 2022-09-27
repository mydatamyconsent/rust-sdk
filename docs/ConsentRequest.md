# ConsentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Consent request id. | 
**template_id** | Option<**String**> | Consent request template id. | [optional]
**consent_id** | Option<**String**> | Consent id. | [optional]
**title** | **String** | Consent title. | 
**description** | **String** | Consent description. | 
**purpose** | Option<**String**> | Consent purpose. | [optional]
**data_life** | Option<[**crate::models::Life**](Life.md)> |  | [optional]
**collectables** | [**Vec<crate::models::CollectibleTypes>**](CollectibleTypes.md) | List of supported collectables. | 
**receiver** | [**crate::models::ConsentRequestReceiver**](ConsentRequestReceiver.md) |  | 
**status** | [**crate::models::DataConsentStatus**](DataConsentStatus.md) |  | 
**created_at_utc** | **String** | Request creation datetime in UTC timezone. | 
**expires_at_utc** | **String** | Request expiration datetime in UTC timezone. | 
**approved_at_utc** | Option<**String**> | Request approval datetime in UTC timezone. | [optional]
**data_access_expires_at_utc** | Option<**String**> | Data access expiration datetime in UTC timezone. | [optional]
**rejected_at_utc** | Option<**String**> | Request rejection datetime in UTC timezone. | [optional]
**revoked_at_utc** | Option<**String**> | Request revocation datetime in UTC timezone. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


