# MyDataMyConsent\DataProcessingAgreementsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_data_agreements_get**](DataProcessingAgreementsApi.md#v1_data_agreements_get) | **GET** /v1/data-agreements | Get paginated data processing agreements.
[**v1_data_agreements_id_delete**](DataProcessingAgreementsApi.md#v1_data_agreements_id_delete) | **DELETE** /v1/data-agreements/{id} | Delete a data processing agreement this will not delete a published or a agreement in use with consents.
[**v1_data_agreements_id_get**](DataProcessingAgreementsApi.md#v1_data_agreements_id_get) | **GET** /v1/data-agreements/{id} | Get data processing agreement by id.
[**v1_data_agreements_id_put**](DataProcessingAgreementsApi.md#v1_data_agreements_id_put) | **PUT** /v1/data-agreements/{id} | Update data processing agreement.
[**v1_data_agreements_id_terminate_put**](DataProcessingAgreementsApi.md#v1_data_agreements_id_terminate_put) | **PUT** /v1/data-agreements/{id}/terminate | Terminate a data processing agreement by id.
[**v1_data_agreements_post**](DataProcessingAgreementsApi.md#v1_data_agreements_post) | **POST** /v1/data-agreements | Create a data processing agreement.



## v1_data_agreements_get

> crate::models::PaginatedListOfDataProcessingAgreements v1_data_agreements_get(page_no, page_size)
Get paginated data processing agreements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfDataProcessingAgreements**](PaginatedListOfDataProcessingAgreements.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_data_agreements_id_delete

> bool v1_data_agreements_id_delete(id)
Delete a data processing agreement this will not delete a published or a agreement in use with consents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_data_agreements_id_get

> crate::models::DataProcessingAgreement v1_data_agreements_id_get(id)
Get data processing agreement by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DataProcessingAgreement**](DataProcessingAgreement.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_data_agreements_id_put

> crate::models::DataProcessingAgreement v1_data_agreements_id_put(id, update_data_processing_agreement)
Update data processing agreement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_data_processing_agreement** | [**UpdateDataProcessingAgreement**](UpdateDataProcessingAgreement.md) |  | [required] |

### Return type

[**crate::models::DataProcessingAgreement**](DataProcessingAgreement.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_data_agreements_id_terminate_put

> bool v1_data_agreements_id_terminate_put(id)
Terminate a data processing agreement by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_data_agreements_post

> crate::models::DataProcessingAgreement v1_data_agreements_post(create_data_processing_agreement)
Create a data processing agreement.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_data_processing_agreement** | [**CreateDataProcessingAgreement**](CreateDataProcessingAgreement.md) |  | [required] |

### Return type

[**crate::models::DataProcessingAgreement**](DataProcessingAgreement.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

