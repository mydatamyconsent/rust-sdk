# MyDataMyConsent\DigiLockerCompatIssuerApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**digilocker_compat_issue_document**](DigiLockerCompatIssuerApi.md#digilocker_compat_issue_document) | **POST** /issuer/issuedoc/1/xml | Digilocker Compatible endpoint to issue document.



## digilocker_compat_issue_document

> crate::models::PushUriResponse digilocker_compat_issue_document(push_uri_request)
Digilocker Compatible endpoint to issue document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_uri_request** | [**PushUriRequest**](PushUriRequest.md) | Push URI request payload | [required] |

### Return type

[**crate::models::PushUriResponse**](PushUriResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml
- **Accept**: application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

