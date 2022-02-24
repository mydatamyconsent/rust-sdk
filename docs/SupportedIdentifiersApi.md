# MyDataMyConsent\SupportedIdentifiersApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_supported_identifiers**](SupportedIdentifiersApi.md#get_all_supported_identifiers) | **GET** /v1/supported-identifiers/{countryIso2Code} | Get all supported identifiers by country.



## get_all_supported_identifiers

> crate::models::SupportedIdentifier get_all_supported_identifiers(country_iso2_code)
Get all supported identifiers by country.

Get all supported identifiers by country.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_iso2_code** | **String** | Country ISO 2 code. | [required] |

### Return type

[**crate::models::SupportedIdentifier**](SupportedIdentifier.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

