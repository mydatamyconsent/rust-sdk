# GetConsentTemplateDetailsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**consent_purpose** | Option<**String**> |  | [optional]
**collectables** | Option<[**Vec<crate::models::CollectibleTypes>**](CollectibleTypes.md)> |  | [optional]
**fetch_type** | Option<[**crate::models::FetchTypes**](FetchTypes.md)> |  | [optional]
**short_id** | Option<**String**> |  | [optional]
**created_by** | Option<**String**> |  | [optional]
**created_at_utc** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**template_type** | Option<[**crate::models::ConsentTemplateTypes**](ConsentTemplateTypes.md)> |  | [optional]
**data_life** | Option<[**crate::models::Life**](Life.md)> |  | [optional]
**request_life** | Option<[**crate::models::Life**](Life.md)> |  | [optional]
**frequency** | Option<[**crate::models::Life**](Life.md)> |  | [optional]
**identity** | Option<[**Vec<crate::models::IdentitySupportedFields>**](IdentitySupportedFields.md)> |  | [optional]
**documents** | Option<[**Vec<crate::models::Document>**](Document.md)> |  | [optional]
**financials** | Option<[**Vec<crate::models::Financial>**](Financial.md)> |  | [optional]
**health_records** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**approved_by** | Option<**String**> |  | [optional]
**approved_at_utc** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


