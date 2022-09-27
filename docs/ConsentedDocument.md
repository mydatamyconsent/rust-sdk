# ConsentedDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Document id. | 
**name** | **String** | Document name. | 
**category** | **String** | Document category. | 
**identifier** | **String** | Document identifier. | 
**field_title** | **String** | Document field title. | 
**field_slug** | **String** | Document field slug. | 
**issued_at_utc** | **String** | Document issued at datetime in UTC timezone. | 
**expires_at_utc** | Option<**String**> | Document expires at datetime in UTC timezone. | [optional]
**issuer** | [**crate::models::ConsentDocumentIssuer**](ConsentDocumentIssuer.md) |  | 
**digital_signatures** | [**Vec<crate::models::DocumentDigitalSignature>**](DocumentDigitalSignature.md) | Digital signatures. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


