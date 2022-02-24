# DocumentIssueRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Request Id. | 
**document_type_id** | **String** |  | 
**document_type_name** | **String** |  | 
**document_identifier** | **String** |  | 
**status** | Option<[**crate::models::DocumentIssueRequestStatus**](DocumentIssueRequestStatus.md)> |  | [optional]
**description** | **String** |  | 
**receiver** | Option<[**serde_json::Value**](.md)> |  | 
**expires_at_utc** | Option<**String**> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> |  | [optional]
**created_at_utc** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


