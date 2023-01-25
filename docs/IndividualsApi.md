# MyDataMyConsent\IndividualsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**issuer_issuedoc1_xml_post**](IndividualsApi.md#issuer_issuedoc1_xml_post) | **POST** /issuer/issuedoc/1/xml | Digilocker compatible endpoint to issue document.
[**v1_individuals_consent_requests_get**](IndividualsApi.md#v1_individuals_consent_requests_get) | **GET** /v1/individuals/consent-requests | Get all consent requests sent to Individuals.
[**v1_individuals_consent_requests_post**](IndividualsApi.md#v1_individuals_consent_requests_post) | **POST** /v1/individuals/consent-requests | Create individual consent request.
[**v1_individuals_consent_requests_request_id_cancel_put**](IndividualsApi.md#v1_individuals_consent_requests_request_id_cancel_put) | **PUT** /v1/individuals/consent-requests/{request_id}/cancel | Cancel the Individual data request by id.
[**v1_individuals_consent_requests_request_id_get**](IndividualsApi.md#v1_individuals_consent_requests_request_id_get) | **GET** /v1/individuals/consent-requests/{request_id} | Get Individual data consent request by id.
[**v1_individuals_consent_templates_get**](IndividualsApi.md#v1_individuals_consent_templates_get) | **GET** /v1/individuals/consent-templates | Get the paginated list of individual consent templates.
[**v1_individuals_consent_templates_template_id_get**](IndividualsApi.md#v1_individuals_consent_templates_template_id_get) | **GET** /v1/individuals/consent-templates/{template_id} | Get Individual consent template details by consent id.
[**v1_individuals_consents_consent_id_documents_document_id_download_get**](IndividualsApi.md#v1_individuals_consents_consent_id_documents_document_id_download_get) | **GET** /v1/individuals/consents/{consent_id}/documents/{document_id}/download | Download Individual consented document by document id.
[**v1_individuals_consents_consent_id_documents_document_id_get**](IndividualsApi.md#v1_individuals_consents_consent_id_documents_document_id_get) | **GET** /v1/individuals/consents/{consent_id}/documents/{document_id} | Get Individual consented document by document id.
[**v1_individuals_consents_consent_id_documents_get**](IndividualsApi.md#v1_individuals_consents_consent_id_documents_get) | **GET** /v1/individuals/consents/{consent_id}/documents | Get Individual consented document by consent id.
[**v1_individuals_consents_consent_id_financial_accounts_account_id_get**](IndividualsApi.md#v1_individuals_consents_consent_id_financial_accounts_account_id_get) | **GET** /v1/individuals/consents/{consent_id}/financial-accounts/{account_id} | Get individual consented financial account details.
[**v1_individuals_consents_consent_id_financial_accounts_account_id_transactions_get**](IndividualsApi.md#v1_individuals_consents_consent_id_financial_accounts_account_id_transactions_get) | **GET** /v1/individuals/consents/{consent_id}/financial-accounts/{account_id}/transactions | Get individual consented financial account transactions.
[**v1_individuals_consents_consent_id_financial_accounts_get**](IndividualsApi.md#v1_individuals_consents_consent_id_financial_accounts_get) | **GET** /v1/individuals/consents/{consent_id}/financial-accounts | Get all individual consented financial accounts.
[**v1_individuals_consents_consent_id_get**](IndividualsApi.md#v1_individuals_consents_consent_id_get) | **GET** /v1/individuals/consents/{consent_id} | Get Individuals consent details by consent id.
[**v1_individuals_consents_consent_id_health_fhir_bundle_get**](IndividualsApi.md#v1_individuals_consents_consent_id_health_fhir_bundle_get) | **GET** /v1/individuals/consents/{consent_id}/health/fhir/bundle | Get Individual consented Health Records by consent id.
[**v1_individuals_consents_get**](IndividualsApi.md#v1_individuals_consents_get) | **GET** /v1/individuals/consents | Get the paginated list of Individual consents.
[**v1_individuals_documents_issue_issue_request_id_upload_post**](IndividualsApi.md#v1_individuals_documents_issue_issue_request_id_upload_post) | **POST** /v1/individuals/documents/issue/{issue_request_id}/upload | Upload a document for issuance request of individual.
[**v1_individuals_documents_issue_post**](IndividualsApi.md#v1_individuals_documents_issue_post) | **POST** /v1/individuals/documents/issue | Issue a new document to an individual user.
[**v1_individuals_documents_issued_document_id_get**](IndividualsApi.md#v1_individuals_documents_issued_document_id_get) | **GET** /v1/individuals/documents/issued/{document_id} | Get issued document.
[**v1_individuals_documents_issued_get**](IndividualsApi.md#v1_individuals_documents_issued_get) | **GET** /v1/individuals/documents/issued | Get paginated list of issued documents of given document type.
[**v1_individuals_documents_types_get**](IndividualsApi.md#v1_individuals_documents_types_get) | **GET** /v1/individuals/documents/types | Get paginated list of registered document types.



## issuer_issuedoc1_xml_post

> crate::models::PushUriResponse issuer_issuedoc1_xml_post(push_uri_request)
Digilocker compatible endpoint to issue document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_uri_request** | [**PushUriRequest**](PushUriRequest.md) |  | [required] |

### Return type

[**crate::models::PushUriResponse**](PushUriResponse.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consent_requests_get

> crate::models::PaginatedListOfIndividualConsentRequestDetailss v1_individuals_consent_requests_get(status, from_date_time, to_date_time, page_no, page_size)
Get all consent requests sent to Individuals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**DataConsentStatus**](.md)> |  |  |
**from_date_time** | Option<**String**> |  |  |
**to_date_time** | Option<**String**> |  |  |
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfIndividualConsentRequestDetailss**](PaginatedListOfIndividualConsentRequestDetailss.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consent_requests_post

> crate::models::IndividualConsentRequestDetails v1_individuals_consent_requests_post(create_consent_request)
Create individual consent request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_consent_request** | [**CreateConsentRequest**](CreateConsentRequest.md) |  | [required] |

### Return type

[**crate::models::IndividualConsentRequestDetails**](IndividualConsentRequestDetails.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consent_requests_request_id_cancel_put

> bool v1_individuals_consent_requests_request_id_cancel_put(request_id)
Cancel the Individual data request by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consent_requests_request_id_get

> crate::models::ConsentRequest v1_individuals_consent_requests_request_id_get(request_id)
Get Individual data consent request by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |

### Return type

[**crate::models::ConsentRequest**](ConsentRequest.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consent_templates_get

> crate::models::PaginatedListOfConsentRequestTemplates v1_individuals_consent_templates_get(page_no, page_size)
Get the paginated list of individual consent templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfConsentRequestTemplates**](PaginatedListOfConsentRequestTemplates.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consent_templates_template_id_get

> crate::models::IndividualConsentRequestTemplateDetails v1_individuals_consent_templates_template_id_get(template_id)
Get Individual consent template details by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |

### Return type

[**crate::models::IndividualConsentRequestTemplateDetails**](IndividualConsentRequestTemplateDetails.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_consent_id_documents_document_id_download_get

> std::path::PathBuf v1_individuals_consents_consent_id_documents_document_id_download_get(consent_id, document_id)
Download Individual consented document by document id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |
**document_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_consent_id_documents_document_id_get

> crate::models::ConsentedDocument v1_individuals_consents_consent_id_documents_document_id_get(consent_id, document_id)
Get Individual consented document by document id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |
**document_id** | **String** |  | [required] |

### Return type

[**crate::models::ConsentedDocument**](ConsentedDocument.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_consent_id_documents_get

> Vec<crate::models::ConsentedDocument> v1_individuals_consents_consent_id_documents_get(consent_id)
Get Individual consented document by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::ConsentedDocument>**](ConsentedDocument.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_consent_id_financial_accounts_account_id_get

> crate::models::FinancialAccount v1_individuals_consents_consent_id_financial_accounts_account_id_get(consent_id, account_id)
Get individual consented financial account details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |
**account_id** | **String** |  | [required] |

### Return type

[**crate::models::FinancialAccount**](FinancialAccount.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_consent_id_financial_accounts_account_id_transactions_get

> crate::models::PaginatedListOfFinancialAccountTransactions v1_individuals_consents_consent_id_financial_accounts_account_id_transactions_get(consent_id, account_id, _filters, _from_date_time, _to_date_time, _page_no, _page_size)
Get individual consented financial account transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |
**account_id** | **String** |  | [required] |
**_filters** | Option<**String**> |  |  |
**_from_date_time** | Option<**String**> |  |  |
**_to_date_time** | Option<**String**> |  |  |
**_page_no** | Option<**i32**> |  |  |
**_page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfFinancialAccountTransactions**](PaginatedListOfFinancialAccountTransactions.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_consent_id_financial_accounts_get

> Vec<crate::models::FinancialAccount> v1_individuals_consents_consent_id_financial_accounts_get(consent_id)
Get all individual consented financial accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::FinancialAccount>**](FinancialAccount.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_consent_id_get

> crate::models::ConsentDetails v1_individuals_consents_consent_id_get(consent_id)
Get Individuals consent details by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |

### Return type

[**crate::models::ConsentDetails**](ConsentDetails.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_consent_id_health_fhir_bundle_get

> crate::models::FhirBundleLessThanAnyGreaterThan v1_individuals_consents_consent_id_health_fhir_bundle_get(consent_id)
Get Individual consented Health Records by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |

### Return type

[**crate::models::FhirBundleLessThanAnyGreaterThan**](FhirBundle<any>.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_consents_get

> crate::models::PaginatedListOfConsents v1_individuals_consents_get(status, from_date_time, to_date_time, page_no, page_size)
Get the paginated list of Individual consents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**DataConsentStatus**](.md)> |  |  |
**from_date_time** | Option<**String**> |  |  |
**to_date_time** | Option<**String**> |  |  |
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfConsents**](PaginatedListOfConsents.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_documents_issue_issue_request_id_upload_post

> crate::models::UploadDocumentResponse v1_individuals_documents_issue_issue_request_id_upload_post(issue_request_id, file)
Upload a document for issuance request of individual.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_request_id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::UploadDocumentResponse**](UploadDocumentResponse.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_documents_issue_post

> crate::models::DocumentIssueRequestDetails v1_individuals_documents_issue_post(document_issue_request)
Issue a new document to an individual user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_issue_request** | [**DocumentIssueRequest**](DocumentIssueRequest.md) |  | [required] |

### Return type

[**crate::models::DocumentIssueRequestDetails**](DocumentIssueRequestDetails.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_documents_issued_document_id_get

> crate::models::IssuedDocument v1_individuals_documents_issued_document_id_get(document_id)
Get issued document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** |  | [required] |

### Return type

[**crate::models::IssuedDocument**](IssuedDocument.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_documents_issued_get

> crate::models::PaginatedListOfIssuedDocuments v1_individuals_documents_issued_get(document_type_id, from_date_time, to_date_time, page_no, page_size)
Get paginated list of issued documents of given document type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_type_id** | **String** |  | [required] |
**from_date_time** | Option<**String**> |  |  |
**to_date_time** | Option<**String**> |  |  |
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfIssuedDocuments**](PaginatedListOfIssuedDocuments.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_individuals_documents_types_get

> crate::models::PaginatedListOfDocumentTypes v1_individuals_documents_types_get(page_no, page_size)
Get paginated list of registered document types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfDocumentTypes**](PaginatedListOfDocumentTypes.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

