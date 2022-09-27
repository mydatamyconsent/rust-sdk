# DocumentIssueRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document_type_id** | **String** | Document type id. | 
**identifier** | **String** | Document identifier. | 
**description** | **String** | Document description. | 
**receiver** | [**crate::models::DocumentIssueRequestReceiver**](DocumentIssueRequest_receiver.md) |  | 
**payment_request** | Option<[**crate::models::DocumentIssueRequestPaymentRequest**](DocumentIssueRequest_paymentRequest.md)> |  | [optional]
**issued_at_utc** | **String** | Datetime of issue in UTC timezone. | 
**valid_from_utc** | **String** | Valid from datetime in UTC timezone. | 
**expires_at_utc** | Option<**String**> | Datetime of expiry in UTC timezone. | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> | Metadata. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


