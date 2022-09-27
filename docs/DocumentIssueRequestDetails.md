# DocumentIssueRequestDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Document issue request Id. | 
**document_type_id** | **String** | Document type Id. | 
**type_name** | **String** | Document type name. | 
**identifier** | **String** | Document identifier. | 
**status** | [**crate::models::DocumentIssueRequestStatus**](DocumentIssueRequestStatus.md) |  | 
**description** | **String** | Document description. | 
**receiver** | [**crate::models::DocumentIssueRequestDetailsReceiver**](DocumentIssueRequestDetails_receiver.md) |  | 
**payment_request** | Option<[**crate::models::PaymentRequest**](PaymentRequest.md)> |  | [optional]
**issued_at_utc** | **String** | Datetime of issue in UTC timezone. | 
**valid_from_utc** | **String** | Valid from datetime in UTC timezone. | 
**expires_at_utc** | Option<**String**> | Datetime of expiry in UTC timezone. | [optional]
**meta_data** | Option<[**serde_json::Value**](.md)> | Metadata. | [optional]
**created_at_utc** | **String** | Creation datetime of issue request in UTC timezone. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


