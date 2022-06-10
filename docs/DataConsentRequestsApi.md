# MyDataMyConsent\DataConsentRequestsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_individual_data_consent_request**](DataConsentRequestsApi.md#cancel_individual_data_consent_request) | **PUT** /v1/consent-requests/individual/{requestId}/cancel | Cancel the individual data consent request by Id.
[**cancel_organization_data_consent_request**](DataConsentRequestsApi.md#cancel_organization_data_consent_request) | **PUT** /v1/consent-requests/organization/{requestId}/cancel | Cancel the organization data consent request by Id.
[**create_individual_data_consent_request**](DataConsentRequestsApi.md#create_individual_data_consent_request) | **POST** /v1/consent-requests/individual | Create data consent request for an individual.
[**create_organization_data_consent_request**](DataConsentRequestsApi.md#create_organization_data_consent_request) | **POST** /v1/consent-requests/organization | Create data consent request for an organization.
[**get_all_consent_requests_to_individuals**](DataConsentRequestsApi.md#get_all_consent_requests_to_individuals) | **GET** /v1/consent-requests/individuals | Get all Consent Requests sent to individuals.
[**get_all_consent_requests_to_organizations**](DataConsentRequestsApi.md#get_all_consent_requests_to_organizations) | **GET** /v1/consent-requests/organizations | Get all Consent Requests sent to organizations.
[**get_individual_consent_request_by_id**](DataConsentRequestsApi.md#get_individual_consent_request_by_id) | **GET** /v1/consent-requests/individuals/{requestId} | Get individual data consent request by id.
[**get_organization_consent_request_by_id**](DataConsentRequestsApi.md#get_organization_consent_request_by_id) | **GET** /v1/consent-requests/organizations/{requestId} | Get a OrganizationConsent Request by Id.



## cancel_individual_data_consent_request

> cancel_individual_data_consent_request(request_id)
Cancel the individual data consent request by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Individual consent request id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_organization_data_consent_request

> cancel_organization_data_consent_request(request_id)
Cancel the organization data consent request by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Organization consent request id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_individual_data_consent_request

> crate::models::IndividualDataConsentRequestDetails create_individual_data_consent_request(create_data_consent_request)
Create data consent request for an individual.

Create data consent request for an individual.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_data_consent_request** | [**CreateDataConsentRequest**](CreateDataConsentRequest.md) | The Individual data consent request payload | [required] |

### Return type

[**crate::models::IndividualDataConsentRequestDetails**](IndividualDataConsentRequestDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization_data_consent_request

> crate::models::OrganizationDataConsentRequestDetails create_organization_data_consent_request(create_data_consent_request)
Create data consent request for an organization.

Create data consent request for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_data_consent_request** | [**CreateDataConsentRequest**](CreateDataConsentRequest.md) | The Organization data consent request payload | [required] |

### Return type

[**crate::models::OrganizationDataConsentRequestDetails**](OrganizationDataConsentRequestDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_consent_requests_to_individuals

> crate::models::IndividualDataConsentRequestDetailsPaginatedList get_all_consent_requests_to_individuals(status, start_date_time, end_date_time, page_no, page_size)
Get all Consent Requests sent to individuals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**DataConsentStatus**](.md)> | Data consent status. |  |
**start_date_time** | Option<**String**> | Start datetime in UTC timezone. |  |
**end_date_time** | Option<**String**> | End datetime in UTC timezone. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::IndividualDataConsentRequestDetailsPaginatedList**](IndividualDataConsentRequestDetailsPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_consent_requests_to_organizations

> crate::models::OrganizationDataConsentRequestDetailsPaginatedList get_all_consent_requests_to_organizations(status, start_date_time, end_date_time, page_no, page_size)
Get all Consent Requests sent to organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**DataConsentStatus**](.md)> | Data consent status. |  |
**start_date_time** | Option<**String**> | Start datetime in UTC timezone. |  |
**end_date_time** | Option<**String**> | End datetime in UTC timezone. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::OrganizationDataConsentRequestDetailsPaginatedList**](OrganizationDataConsentRequestDetailsPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_individual_consent_request_by_id

> crate::models::DataConsentRequest get_individual_consent_request_by_id(request_id)
Get individual data consent request by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Individual data consent request id. | [required] |

### Return type

[**crate::models::DataConsentRequest**](DataConsentRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_consent_request_by_id

> crate::models::DataConsentRequest get_organization_consent_request_by_id(request_id)
Get a OrganizationConsent Request by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Organization consent request id. | [required] |

### Return type

[**crate::models::DataConsentRequest**](DataConsentRequest.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

