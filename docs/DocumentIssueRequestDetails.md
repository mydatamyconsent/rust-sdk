# DocumentIssueRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Request Id. | 
**document_type_id** | **String** |  | 
**type_name** | **String** |  | 
**identifier** | **String** |  | 
**status** | [**crate::models::DocumentIssueRequestStatus**](DocumentIssueRequestStatus.md) |  | 
**description** | **String** |  | 
**receiver** | Option<[**serde_json::Value**](.md)> |  | 
**issued_at_utc** | **String** |  | 
**valid_from_utc** | **String** |  | 
**expires_at_utc** | Option<**String**> |  | [optional]
**meta_data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created_at_utc** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


