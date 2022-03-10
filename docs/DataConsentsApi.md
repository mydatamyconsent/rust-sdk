# MyDataMyConsent\DataConsentsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_consented_document_analysis**](DataConsentsApi.md#download_consented_document_analysis) | **GET** /v1/consents/{consentId}/documents/{documentId}/analysis | Get analysis of a consented document.
[**download_consented_document_by_id**](DataConsentsApi.md#download_consented_document_by_id) | **GET** /v1/consents/individuals/{consentId}/documents/{documentId}/download | Download a individuals consented document.
[**download_org_consented_document_by_id**](DataConsentsApi.md#download_org_consented_document_by_id) | **GET** /v1/consents/organizations/{consentId}/documents/{documentId}/download | Download a organizations consented document.
[**get_all_consented_documents**](DataConsentsApi.md#get_all_consented_documents) | **GET** /v1/consents/individuals/{consentId}/documents | Get the individual documents based on ConsentId.
[**get_all_consented_financial_accounts**](DataConsentsApi.md#get_all_consented_financial_accounts) | **GET** /v1/consents/individuals/{consentId}/financial-accounts | Get all individual consented financial accounts.
[**get_all_organization_consented_documents**](DataConsentsApi.md#get_all_organization_consented_documents) | **GET** /v1/consents/organizations/{consentId}/documents | Get the organization documents based on ConsentId.
[**get_consent_details_by_id**](DataConsentsApi.md#get_consent_details_by_id) | **GET** /v1/consents/individuals/{consentId} | Get all individuals consent details by consent id.
[**get_consent_financial_accounts**](DataConsentsApi.md#get_consent_financial_accounts) | **GET** /v1/consents/organizations/{consentId}/financial-accounts | Get all organizational consented financial accounts.
[**get_consented_account_by_id**](DataConsentsApi.md#get_consented_account_by_id) | **GET** /v1/consents/individuals/{consentId}/financial-accounts/{accountId} | Get individual consented financial account details based on account id.
[**get_consented_document_by_id**](DataConsentsApi.md#get_consented_document_by_id) | **GET** /v1/consents/individuals/{consentId}/documents/{documentId} | Get individuals consent document based on document id.
[**get_consented_financial_account**](DataConsentsApi.md#get_consented_financial_account) | **GET** /v1/consents/organizations/{consentId}/financial-accounts/{accountId} | Get organization consented financial account details based on account id.
[**get_consented_financial_account_insights**](DataConsentsApi.md#get_consented_financial_account_insights) | **GET** /v1/consents/{consentId}/financial-accounts/{accountId}/insights | Get consented financial account insights.
[**get_consented_financial_account_transactions**](DataConsentsApi.md#get_consented_financial_account_transactions) | **GET** /v1/consents/individuals/{consentId}/financial-accounts/{accountId}/transactions | Get individual consented financial account transactions of an individual based on accountId.
[**get_consents_for_organizations**](DataConsentsApi.md#get_consents_for_organizations) | **GET** /v1/consents/organizations | Get the list of data consents sent for organizations.
[**get_consents_sent_to_individuals**](DataConsentsApi.md#get_consents_sent_to_individuals) | **GET** /v1/consents/individuals | Get the list of Consents Sent to Individuals.
[**get_org_consented_account_transactions**](DataConsentsApi.md#get_org_consented_account_transactions) | **GET** /v1/consents/organizations/{consentId}/financial-accounts/{accountId}/transactions | Get organization consented financial account transactions of an individual based on accountId.
[**get_organization_consent_details_by_id**](DataConsentsApi.md#get_organization_consent_details_by_id) | **GET** /v1/consents/organizations/{consentId} | Get all organization consent details by consent id.
[**get_organization_consented_document_by_id**](DataConsentsApi.md#get_organization_consented_document_by_id) | **GET** /v1/consents/organizations/{consentId}/documents/{documentId} | Get organization consent document based on document id.



## download_consented_document_analysis

> download_consented_document_analysis(consent_id, document_id)
Get analysis of a consented document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |
**document_id** | **String** | Document Id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_consented_document_by_id

> crate::models::UserDocumentDownload download_consented_document_by_id(consent_id, document_id)
Download a individuals consented document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |
**document_id** | **String** | Document id. | [required] |

### Return type

[**crate::models::UserDocumentDownload**](UserDocumentDownload.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_org_consented_document_by_id

> crate::models::OrganizationDocumentDownloadDto download_org_consented_document_by_id(consent_id, document_id)
Download a organizations consented document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |
**document_id** | **String** | Document id. | [required] |

### Return type

[**crate::models::OrganizationDocumentDownloadDto**](OrganizationDocumentDownloadDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_consented_documents

> crate::models::DataConsentDocumentsDto get_all_consented_documents(consent_id)
Get the individual documents based on ConsentId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |

### Return type

[**crate::models::DataConsentDocumentsDto**](DataConsentDocumentsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_consented_financial_accounts

> crate::models::DataConsentFinancialsDto get_all_consented_financial_accounts(consent_id)
Get all individual consented financial accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |

### Return type

[**crate::models::DataConsentFinancialsDto**](DataConsentFinancialsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_organization_consented_documents

> crate::models::DataConsentDocumentsDto get_all_organization_consented_documents(consent_id)
Get the organization documents based on ConsentId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |

### Return type

[**crate::models::DataConsentDocumentsDto**](DataConsentDocumentsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consent_details_by_id

> crate::models::DataConsentDetailsDto get_consent_details_by_id(consent_id)
Get all individuals consent details by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |

### Return type

[**crate::models::DataConsentDetailsDto**](DataConsentDetailsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consent_financial_accounts

> crate::models::DataConsentFinancialsDto get_consent_financial_accounts(consent_id)
Get all organizational consented financial accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |

### Return type

[**crate::models::DataConsentFinancialsDto**](DataConsentFinancialsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consented_account_by_id

> crate::models::FinancialAccount get_consented_account_by_id(consent_id, account_id)
Get individual consented financial account details based on account id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |
**account_id** | **String** | Account id. | [required] |

### Return type

[**crate::models::FinancialAccount**](FinancialAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consented_document_by_id

> crate::models::UserDocumentDetails get_consented_document_by_id(consent_id, document_id)
Get individuals consent document based on document id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |
**document_id** | **String** | Document Id. | [required] |

### Return type

[**crate::models::UserDocumentDetails**](UserDocumentDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consented_financial_account

> crate::models::OrganizationFinancialAccountDto get_consented_financial_account(consent_id, account_id)
Get organization consented financial account details based on account id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |
**account_id** | **String** | Account id. | [required] |

### Return type

[**crate::models::OrganizationFinancialAccountDto**](OrganizationFinancialAccountDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consented_financial_account_insights

> get_consented_financial_account_insights(consent_id, account_id)
Get consented financial account insights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |
**account_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consented_financial_account_transactions

> crate::models::UserAccountFinancialTransactionsDtoPaginatedList get_consented_financial_account_transactions(consent_id, account_id, filters, from_date_time_utc, to_date_time_utc, page_no, page_size)
Get individual consented financial account transactions of an individual based on accountId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |
**account_id** | **String** | Account id. | [required] |
**filters** | Option<**String**> | Filters. |  |
**from_date_time_utc** | Option<**String**> | From date time in utc timezone. |  |
**to_date_time_utc** | Option<**String**> | Til date time in utc timezone. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 10]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::UserAccountFinancialTransactionsDtoPaginatedList**](UserAccountFinancialTransactionsDtoPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_consents_for_organizations

> crate::models::OrganizationDataConsentInfoDtoPaginatedList get_consents_for_organizations(status, from, to, page_no, page_size)
Get the list of data consents sent for organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**crate::models::DataConsentStatus**](.md)> | Data consent status MyDataMyConsent.Domain.Entities.ConsentAggregate.Enums.DataConsentStatus. |  |
**from** | Option<**String**> | From date time in utc timezone. |  |
**to** | Option<**String**> | Til date time in utc timezone. |  |
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


## get_consents_sent_to_individuals

> crate::models::UserDataConsentInfoDtoPaginatedList get_consents_sent_to_individuals(status, from, to, page_no, page_size)
Get the list of Consents Sent to Individuals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**crate::models::DataConsentStatus**](.md)> | Data consent status MyDataMyConsent.Domain.Entities.ConsentAggregate.Enums.DataConsentStatus. |  |
**from** | Option<**String**> | From date time in utc timezone. |  |
**to** | Option<**String**> | Til date time in utc timezone. |  |
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


## get_org_consented_account_transactions

> crate::models::OrganizationFinancialTransactionsDtoPaginatedList get_org_consented_account_transactions(consent_id, account_id, filters, from_date_time_utc, to_date_time_utc, page_no, page_size)
Get organization consented financial account transactions of an individual based on accountId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |
**account_id** | **String** | Account id. | [required] |
**filters** | Option<**String**> | Filters. |  |
**from_date_time_utc** | Option<**String**> | From date time in utc timezone. |  |
**to_date_time_utc** | Option<**String**> | Til date time in utc timezone. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::OrganizationFinancialTransactionsDtoPaginatedList**](OrganizationFinancialTransactionsDtoPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_consent_details_by_id

> crate::models::DataConsentDetailsDto get_organization_consent_details_by_id(consent_id)
Get all organization consent details by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |

### Return type

[**crate::models::DataConsentDetailsDto**](DataConsentDetailsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_consented_document_by_id

> crate::models::OrganizationDocumentDetails get_organization_consented_document_by_id(consent_id, document_id)
Get organization consent document based on document id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Consent id. | [required] |
**document_id** | **String** | Document Id. | [required] |

### Return type

[**crate::models::OrganizationDocumentDetails**](OrganizationDocumentDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

