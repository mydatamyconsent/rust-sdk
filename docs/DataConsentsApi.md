# MyDataMyConsent\DataConsentsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_consented_document_analysis**](DataConsentsApi.md#download_consented_document_analysis) | **GET** /v1/consents/{consentId}/documents/{documentId}/analysis | Get analysis of a consented document.
[**download_individual_consented_document_by_id**](DataConsentsApi.md#download_individual_consented_document_by_id) | **GET** /v1/consents/individuals/{consentId}/documents/{documentId}/download | Download individual consented document by document id.
[**download_organization_consented_document_by_id**](DataConsentsApi.md#download_organization_consented_document_by_id) | **GET** /v1/consents/organizations/{consentId}/documents/{documentId}/download | Download organization consent document based on document id.
[**get_all_consented_financial_accounts**](DataConsentsApi.md#get_all_consented_financial_accounts) | **GET** /v1/consents/individuals/{consentId}/financial-accounts | Get all individual consented financial accounts.
[**get_consent_financial_accounts**](DataConsentsApi.md#get_consent_financial_accounts) | **GET** /v1/consents/organizations/{consentId}/financial-accounts | Get all organizational consented financial accounts.
[**get_consented_account_by_id**](DataConsentsApi.md#get_consented_account_by_id) | **GET** /v1/consents/individuals/{consentId}/financial-accounts/{accountId} | Get individual consented financial account details based on account id.
[**get_consented_document_by_id**](DataConsentsApi.md#get_consented_document_by_id) | **GET** /v1/consents/individuals/{consentId}/documents/{documentId} | Get individual consented document by document id.
[**get_consented_financial_account**](DataConsentsApi.md#get_consented_financial_account) | **GET** /v1/consents/organizations/{consentId}/financial-accounts/{accountId} | Get organization consented financial account details based on account id.
[**get_consented_financial_account_insights**](DataConsentsApi.md#get_consented_financial_account_insights) | **GET** /v1/consents/{consentId}/financial-accounts/{accountId}/insights | Get consented financial account insights.
[**get_consented_financial_account_transactions**](DataConsentsApi.md#get_consented_financial_account_transactions) | **GET** /v1/consents/individuals/{consentId}/financial-accounts/{accountId}/transactions | Get individual consented financial account transactions of an individual based on accountId.
[**get_consents**](DataConsentsApi.md#get_consents) | **GET** /v1/consents/individuals | Get the paginated list of individual data consents.
[**get_individual_consented_documents**](DataConsentsApi.md#get_individual_consented_documents) | **GET** /v1/consents/individuals/{consentId}/documents | Get individual consented documents by consent id.
[**get_individual_data_consent_by_id**](DataConsentsApi.md#get_individual_data_consent_by_id) | **GET** /v1/consents/individuals/{consentId} | Get individuals data consent details by consent id.
[**get_org_consented_account_transactions**](DataConsentsApi.md#get_org_consented_account_transactions) | **GET** /v1/consents/organizations/{consentId}/financial-accounts/{accountId}/transactions | Get organization consented financial account transactions of an individual based on accountId.
[**get_organization_consented_document_by_id**](DataConsentsApi.md#get_organization_consented_document_by_id) | **GET** /v1/consents/organizations/{consentId}/documents/{documentId} | Get organization consent document based on document id.
[**get_organization_consented_documents**](DataConsentsApi.md#get_organization_consented_documents) | **GET** /v1/consents/organizations/{consentId}/documents | Get organization consented documents by consent id.
[**get_organization_data_consent_by_id**](DataConsentsApi.md#get_organization_data_consent_by_id) | **GET** /v1/consents/organizations/{consentId} | Get organizations data consent details by consent id.
[**get_organization_data_consents**](DataConsentsApi.md#get_organization_data_consents) | **GET** /v1/consents/organizations | Get the paginated list of organization data consents.



## download_consented_document_analysis

> download_consented_document_analysis(consent_id, document_id)
Get analysis of a consented document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Data consent id. | [required] |
**document_id** | **String** | Consented document Id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_individual_consented_document_by_id

> download_individual_consented_document_by_id(consent_id, document_id)
Download individual consented document by document id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Individual data consent id. | [required] |
**document_id** | **String** | Consented document id. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_organization_consented_document_by_id

> download_organization_consented_document_by_id(consent_id, document_id)
Download organization consent document based on document id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Organization data consent id. | [required] |
**document_id** | **String** | Organization consented document Id. | [required] |

### Return type

 (empty response body)

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

> crate::models::DataConsentDocument get_consented_document_by_id(consent_id, document_id)
Get individual consented document by document id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Individual data consent id. | [required] |
**document_id** | **String** | Consented document id. | [required] |

### Return type

[**crate::models::DataConsentDocument**](DataConsentDocument.md)

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


## get_consents

> crate::models::DataConsentDetailsPaginatedList get_consents(status, from_date_time, to_date_time, page_no, page_size)
Get the paginated list of individual data consents.

GetIndividualDataConsents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**DataConsentStatus**](.md)> | Data consent status. |  |
**from_date_time** | Option<**String**> | From datetime in UTC timezone. |  |
**to_date_time** | Option<**String**> | To datetime in UTC timezone. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::DataConsentDetailsPaginatedList**](DataConsentDetailsPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_individual_consented_documents

> Vec<crate::models::DataConsentDocument> get_individual_consented_documents(consent_id)
Get individual consented documents by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Individual data consent id. | [required] |

### Return type

[**Vec<crate::models::DataConsentDocument>**](DataConsentDocument.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_individual_data_consent_by_id

> crate::models::DataConsent get_individual_data_consent_by_id(consent_id)
Get individuals data consent details by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Individual data consent id. | [required] |

### Return type

[**crate::models::DataConsent**](DataConsent.md)

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


## get_organization_consented_document_by_id

> crate::models::DataConsentDocument get_organization_consented_document_by_id(consent_id, document_id)
Get organization consent document based on document id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Organization data consent id. | [required] |
**document_id** | **String** | Organization consented document Id. | [required] |

### Return type

[**crate::models::DataConsentDocument**](DataConsentDocument.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_consented_documents

> Vec<crate::models::DataConsentDocument> get_organization_consented_documents(consent_id)
Get organization consented documents by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Organization data consent id. | [required] |

### Return type

[**Vec<crate::models::DataConsentDocument>**](DataConsentDocument.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_data_consent_by_id

> crate::models::DataConsent get_organization_data_consent_by_id(consent_id)
Get organizations data consent details by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** | Organization data consent id. | [required] |

### Return type

[**crate::models::DataConsent**](DataConsent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_data_consents

> crate::models::DataConsentDetailsPaginatedList get_organization_data_consents(status, from_date_time, to_date_time, page_no, page_size)
Get the paginated list of organization data consents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**DataConsentStatus**](.md)> | Data consent status. |  |
**from_date_time** | Option<**String**> | From datetime in UTC timezone. |  |
**to_date_time** | Option<**String**> | To datetime in UTC timezone. |  |
**page_no** | Option<**i32**> | Page number. |  |[default to 1]
**page_size** | Option<**i32**> | Number of items to return. |  |[default to 25]

### Return type

[**crate::models::DataConsentDetailsPaginatedList**](DataConsentDetailsPaginatedList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

