# IssuedDocumentDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**receiver** | [**crate::models::DocumentReceiver**](DocumentReceiver.md) |  | 
**metadata** | Option<**::std::collections::HashMap<String, String>**> | Metadata. | [optional]
**digital_signatures** | [**Vec<crate::models::DocumentDigitalSignature>**](DocumentDigitalSignature.md) | Digital signatures. | 
**id** | **String** | Document Id. | 
**identifier** | **String** | Document Identifier. | 
**document_type** | **String** | Document type name. | 
**issued_to** | **String** | User name. | 
**issued_at_utc** | **String** | Issued datetime in UTC timezone. | 
**expires_at_utc** | Option<**String**> | Expires datetime in UTC timezone. | [optional]
**accepted_at_utc** | Option<**String**> | Accepted datetime in UTC timezone. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


