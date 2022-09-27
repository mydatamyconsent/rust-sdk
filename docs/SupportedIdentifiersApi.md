# MyDataMyConsent\SupportedIdentifiersApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_supported_identifiers_country_iso2_code_get**](SupportedIdentifiersApi.md#v1_supported_identifiers_country_iso2_code_get) | **GET** /v1/supported-identifiers/{country_iso2_code} | Get all supported identifiers by country.



## v1_supported_identifiers_country_iso2_code_get

> crate::models::SupportedIdentifier v1_supported_identifiers_country_iso2_code_get(country_iso2_code)
Get all supported identifiers by country.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_iso2_code** | **String** |  | [required] |

### Return type

[**crate::models::SupportedIdentifier**](SupportedIdentifier.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

