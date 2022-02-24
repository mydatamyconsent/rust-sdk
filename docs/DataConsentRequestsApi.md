# MyDataMyConsent\DataConsentRequestsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_individual_data_consent_request**](DataConsentRequestsApi.md#cancel_individual_data_consent_request) | **PUT** /v1/consent-requests/individual/{requestId}/cancel | Cancel the individual data consent request based on Id.
[**cancel_organization_data_consent_request**](DataConsentRequestsApi.md#cancel_organization_data_consent_request) | **PUT** /v1/consent-requests/organization/{requestId}/cancel | Cancel the Organization data consent request based on Id.
[**create_individual_data_consent_request**](DataConsentRequestsApi.md#create_individual_data_consent_request) | **POST** /v1/consent-requests/individual | Create a individual data consent request.
[**create_organization_data_consent_request**](DataConsentRequestsApi.md#create_organization_data_consent_request) | **POST** /v1/consent-requests/organization | Create a organization data consent request.
[**get_all_consent_requests_to_individuals**](DataConsentRequestsApi.md#get_all_consent_requests_to_individuals) | **GET** /v1/consent-requests/individuals | Get all Consent Requests sent to Individuals.
[**get_all_consent_requests_to_organizations**](DataConsentRequestsApi.md#get_all_consent_requests_to_organizations) | **GET** /v1/consent-requests/organizations | Get All Consent Requests sent to Organizations.
[**get_individual_consent_request_by_id**](DataConsentRequestsApi.md#get_individual_consent_request_by_id) | **GET** /v1/consent-requests/individuals/{requestId} | Get a Consent Request by ID.
[**get_organization_consent_request_by_id**](DataConsentRequestsApi.md#get_organization_consent_request_by_id) | **GET** /v1/consent-requests/organizations/{requestId} | Get a OrganizationConsent Request by Id.



## cancel_individual_data_consent_request

> crate::models::IndividualDataConsentRequestResponse cancel_individual_data_consent_request(request_id)
Cancel the individual data consent request based on Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Individual consent request id. | [required] |

### Return type

[**crate::models::IndividualDataConsentRequestResponse**](IndividualDataConsentRequestResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_organization_data_consent_request

> crate::models::OrganizationDataConsentRequestResponse cancel_organization_data_consent_request(request_id)
Cancel the Organization data consent request based on Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Organization consent request id. | [required] |

### Return type

[**crate::models::OrganizationDataConsentRequestResponse**](OrganizationDataConsentRequestResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_individual_data_consent_request

> crate::models::IndividualDataConsentRequestResponse create_individual_data_consent_request(create_individual_data_consent_request)
Create a individual data consent request.

Create a individual data consent request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_individual_data_consent_request** | [**CreateIndividualDataConsentRequest**](CreateIndividualDataConsentRequest.md) | The Individual data consent request payload | [required] |

### Return type

[**crate::models::IndividualDataConsentRequestResponse**](IndividualDataConsentRequestResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_organization_data_consent_request

> crate::models::OrganizationDataConsentRequestResponse create_organization_data_consent_request(create_organization_data_consent_request)
Create a organization data consent request.

Create a organization data consent request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_organization_data_consent_request** | [**CreateOrganizationDataConsentRequest**](CreateOrganizationDataConsentRequest.md) | M:MyDataMyConsent.DeveloperApi.Controllers.DataConsentRequestsController.CreateOrganizationDataConsentRequest(MyDataMyConsent.DeveloperApi.Models.CreateOrganizationDataConsentRequest). | [required] |

### Return type

[**crate::models::OrganizationDataConsentRequestResponse**](OrganizationDataConsentRequestResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_consent_requests_to_individuals

> crate::models::UserDataConsentInfoDtoPaginatedList get_all_consent_requests_to_individuals(status, start_date_time, end_date_time, page_no, page_size)
Get all Consent Requests sent to Individuals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**crate::models::DataConsentStatus**](.md)> | Data consent status. |  |
**start_date_time** | Option<**String**> | Start date time. |  |
**end_date_time** | Option<**String**> | End date time. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::UserDataConsentInfoDtoPaginatedList**](UserDataConsentInfoDtoPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_consent_requests_to_organizations

> crate::models::OrganizationDataConsentInfoDtoPaginatedList get_all_consent_requests_to_organizations(status, start_date_time, end_date_time, page_no, page_size)
Get All Consent Requests sent to Organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**crate::models::DataConsentStatus**](.md)> | Data consent status. |  |
**start_date_time** | Option<**String**> | Start date time. |  |
**end_date_time** | Option<**String**> | End date time. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::OrganizationDataConsentInfoDtoPaginatedList**](OrganizationDataConsentInfoDtoPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_individual_consent_request_by_id

> crate::models::DataConsentDetailsDto get_individual_consent_request_by_id(request_id)
Get a Consent Request by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Individual consent request id. | [required] |

### Return type

[**crate::models::DataConsentDetailsDto**](DataConsentDetailsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_consent_request_by_id

> crate::models::DataConsentDetailsDto get_organization_consent_request_by_id(request_id)
Get a OrganizationConsent Request by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Organization consent request id. | [required] |

### Return type

[**crate::models::DataConsentDetailsDto**](DataConsentDetailsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

