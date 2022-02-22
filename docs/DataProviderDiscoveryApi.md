# MyDataMyConsent\DataProviderDiscoveryApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_data_provider_by_id**](DataProviderDiscoveryApi.md#get_data_provider_by_id) | **GET** /v1/data-providers/{providerId} | Get a Data Provider details based on provider id.
[**get_data_providers**](DataProviderDiscoveryApi.md#get_data_providers) | **GET** /v1/data-providers | Discover all data providers in My Data My Consent by country and filters.



## get_data_provider_by_id

> crate::models::DataProvider get_data_provider_by_id(provider_id)
Get a Data Provider details based on provider id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_id** | **String** | Provider id. | [required] |

### Return type

[**crate::models::DataProvider**](DataProvider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_providers

> crate::models::DataProviderPaginatedList get_data_providers(account_type, document_type, organization_category, page_no, page_size, country)
Discover all data providers in My Data My Consent by country and filters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_type** | Option<**String**> | Account type. |  |
**document_type** | Option<**String**> | Document type. |  |
**organization_category** | Option<**String**> | Organization category. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]
**country** | Option<**String**> | ISO2 Country code. |  |[default to IN]

### Return type

[**crate::models::DataProviderPaginatedList**](DataProviderPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

