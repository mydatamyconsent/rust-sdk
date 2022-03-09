# DataConsentDetailsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**consent_request_id** | **String** |  | 
**title** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**data_life** | Option<[**crate::models::Life**](Life.md)> |  | [optional]
**requested_by_org** | Option<[**crate::models::Requester**](Requester.md)> |  | [optional]
**collectables** | [**Vec<crate::models::CollectibleTypes>**](CollectibleTypes.md) |  | 
**status** | Option<[**crate::models::DataConsentStatus**](DataConsentStatus.md)> |  | [optional]
**approved_at_utc** | Option<**String**> |  | [optional]
**approved_expires_at_utc** | Option<**String**> |  | [optional]
**rejected_at_utc** | Option<**String**> |  | [optional]
**revoked_at_utc** | Option<**String**> |  | [optional]
**requested_expires_at_utc** | Option<**String**> |  | [optional]
**requested_at_utc** | Option<**String**> |  | [optional]
**identifiers** | Option<[**serde_json::Value**](.md)> |  | [optional]
**documents** | Option<[**Vec<crate::models::DataConsentDocumentDetailsDto>**](DataConsentDocumentDetailsDto.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


