# MyDataMyConsent\DataProvidersDiscoveryApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_data_providers_get**](DataProvidersDiscoveryApi.md#v1_data_providers_get) | **GET** /v1/data-providers | Discover all data providers in my data my consent by country and filters.
[**v1_data_providers_id_get**](DataProvidersDiscoveryApi.md#v1_data_providers_id_get) | **GET** /v1/data-providers/{id} | Get a data provider details by provider id.



## v1_data_providers_get

> crate::models::PaginatedListOfDataProviders v1_data_providers_get(country_iso2_code, page_no, page_size)
Discover all data providers in my data my consent by country and filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_iso2_code** | **String** |  | [required] |
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfDataProviders**](PaginatedListOfDataProviders.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_data_providers_id_get

> crate::models::DataProviderDetails v1_data_providers_id_get(id)
Get a data provider details by provider id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::DataProviderDetails**](DataProviderDetails.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

