# MyDataMyConsent\DocumentsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_issued_document_by_id**](DocumentsApi.md#get_issued_document_by_id) | **GET** /v1/documents/issued/{documentId} | Get issued document.
[**get_issued_documents**](DocumentsApi.md#get_issued_documents) | **GET** /v1/documents/issued | Get paginated list of issued documents of given document type.
[**get_registered_document_types**](DocumentsApi.md#get_registered_document_types) | **GET** /v1/documents/types | Get paginated list of registered document types.
[**issue_document_to_individual**](DocumentsApi.md#issue_document_to_individual) | **POST** /v1/documents/issue/individual | Issue a new document to an individual user.
[**issue_document_to_organization**](DocumentsApi.md#issue_document_to_organization) | **POST** /v1/documents/issue/organization | Issue a new document to an organization.
[**upload_document_for_individual**](DocumentsApi.md#upload_document_for_individual) | **POST** /v1/documents/issue/individual/upload/{issueRequestId} | Upload a document for issuance request of individual.
[**upload_document_for_organization**](DocumentsApi.md#upload_document_for_organization) | **POST** /v1/documents/issue/organization/upload/{issueRequestId} | Upload a document for issuance request of organization.



## get_issued_document_by_id

> crate::models::IssuedDocumentDetails get_issued_document_by_id(document_id)
Get issued document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document id. | [required] |

### Return type

[**crate::models::IssuedDocumentDetails**](IssuedDocumentDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issued_documents

> crate::models::IssuedDocumentPaginatedList get_issued_documents(document_type_id, from_date_time, to_date_time, page_no, page_size)
Get paginated list of issued documents of given document type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_type_id** | Option<**String**> | Document type id. |  |
**from_date_time** | Option<**String**> | From DateTime in UTC timezone. |  |
**to_date_time** | Option<**String**> | To DateTime in UTC timezone. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::IssuedDocumentPaginatedList**](IssuedDocumentPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_registered_document_types

> crate::models::DocumentTypePaginatedList get_registered_document_types(supported_entity_type, page_no, page_size)
Get paginated list of registered document types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supported_entity_type** | Option<[**crate::models::SupportedEntityType**](.md)> | Supported entity type. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::DocumentTypePaginatedList**](DocumentTypePaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_document_to_individual

> crate::models::DocumentIssueRequestDetails issue_document_to_individual(document_issue_request)
Issue a new document to an individual user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_issue_request** | [**DocumentIssueRequest**](DocumentIssueRequest.md) | Document issue request payload | [required] |

### Return type

[**crate::models::DocumentIssueRequestDetails**](DocumentIssueRequestDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_document_to_organization

> crate::models::DocumentIssueRequestDetails issue_document_to_organization(document_issue_request)
Issue a new document to an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_issue_request** | [**DocumentIssueRequest**](DocumentIssueRequest.md) | Document issue request payload | [required] |

### Return type

[**crate::models::DocumentIssueRequestDetails**](DocumentIssueRequestDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_document_for_individual

> upload_document_for_individual(issue_request_id, form_file)
Upload a document for issuance request of individual.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_request_id** | **String** | Document issue request id. | [required] |
**form_file** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_document_for_organization

> upload_document_for_organization(issue_request_id, form_file)
Upload a document for issuance request of organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_request_id** | **String** | Document issue request id System.Guid. | [required] |
**form_file** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

