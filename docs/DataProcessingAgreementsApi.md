# MyDataMyConsent\DataProcessingAgreementsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_data_processing_agreement**](DataProcessingAgreementsApi.md#create_data_processing_agreement) | **POST** /v1/data-agreements | Create a data processing agreement.
[**delete_data_processing_agreement_by_id**](DataProcessingAgreementsApi.md#delete_data_processing_agreement_by_id) | **DELETE** /v1/data-agreements/{id} | Delete a data processing agreement. This will not delete a published or a agreement in use with consents.
[**get_data_processing_agreement_by_id**](DataProcessingAgreementsApi.md#get_data_processing_agreement_by_id) | **GET** /v1/data-agreements/{id} | Get data processing agreement by id.
[**get_data_processing_agreements**](DataProcessingAgreementsApi.md#get_data_processing_agreements) | **GET** /v1/data-agreements | Get paginated data processing agreements.
[**terminate_data_processing_agreement_by_id**](DataProcessingAgreementsApi.md#terminate_data_processing_agreement_by_id) | **PUT** /v1/data-agreements/{id}/terminate | Terminate a data processing agreement.
[**update_data_processing_agreement**](DataProcessingAgreementsApi.md#update_data_processing_agreement) | **PUT** /v1/data-agreements/{id} | Update a data processing agreement.



## create_data_processing_agreement

> crate::models::DataProcessingAgreement create_data_processing_agreement(create_data_processing_agreement)
Create a data processing agreement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_data_processing_agreement** | [**CreateDataProcessingAgreement**](CreateDataProcessingAgreement.md) | Create data processing agreement payload | [required] |

### Return type

[**crate::models::DataProcessingAgreement**](DataProcessingAgreement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_data_processing_agreement_by_id

> delete_data_processing_agreement_by_id(id)
Delete a data processing agreement. This will not delete a published or a agreement in use with consents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Agreement id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_processing_agreement_by_id

> crate::models::DataProcessingAgreement get_data_processing_agreement_by_id(id)
Get data processing agreement by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Agreement id. | [required] |

### Return type

[**crate::models::DataProcessingAgreement**](DataProcessingAgreement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_processing_agreements

> crate::models::DataProcessingAgreementPaginatedList get_data_processing_agreements(page_no, page_size)
Get paginated data processing agreements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::DataProcessingAgreementPaginatedList**](DataProcessingAgreementPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_data_processing_agreement_by_id

> terminate_data_processing_agreement_by_id(id)
Terminate a data processing agreement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Agreement id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_data_processing_agreement

> crate::models::DataProcessingAgreement update_data_processing_agreement(id, update_data_processing_agreement)
Update a data processing agreement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Agreement id. | [required] |
**update_data_processing_agreement** | [**UpdateDataProcessingAgreement**](UpdateDataProcessingAgreement.md) | Update data processing agreement payload | [required] |

### Return type

[**crate::models::DataProcessingAgreement**](DataProcessingAgreement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

