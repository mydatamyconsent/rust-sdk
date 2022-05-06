# DocumentType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Document Type Identifier. | 
**category_type** | [**crate::models::DocumentCategoryType**](DocumentCategoryType.md) |  | 
**sub_category_type** | [**crate::models::DocumentSubCategoryType**](DocumentSubCategoryType.md) |  | 
**name** | **String** | Document Type Name. eg: Driving License. | 
**slug** | **String** | Document Type Unique Slug. eg: \"in.gov.gj.transport.dl\". | 
**description** | Option<**String**> | Document Type description. eg: Gujarat State Driving License. | [optional]
**logo_url** | **String** | Logo URL of document type. | 
**search_service_name** | Option<**String**> | Document search repository service name. | [optional]
**repository_service_name** | Option<**String**> | Document repository service name. | [optional]
**supported_entity_types** | [**Vec<crate::models::SupportedEntityType>**](SupportedEntityType.md) | Supported entity types. eg: Individual, Organization. | 
**added_by** | **String** | Name of the document type creator. | 
**payable_amount** | **f64** | Payable amount if document is chargeable. eg: 10.25. | 
**payable_amount_currency** | Option<**String**> | Payable amount currency. eg: INR, USD etc.,. | [optional]
**approved_at_utc** | Option<**String**> | DateTime of approval in UTC timezone. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


