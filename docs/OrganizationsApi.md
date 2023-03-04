# MyDataMyConsent\OrganizationsApi

All URIs are relative to *https://api.mydatamyconsent.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v1_organizations_consent_requests_get**](OrganizationsApi.md#v1_organizations_consent_requests_get) | **GET** /v1/organizations/consent-requests | Get all consent requests sent to Organizations.
[**v1_organizations_consent_requests_post**](OrganizationsApi.md#v1_organizations_consent_requests_post) | **POST** /v1/organizations/consent-requests | Create consent request for an Organization.
[**v1_organizations_consent_requests_request_id_cancel_put**](OrganizationsApi.md#v1_organizations_consent_requests_request_id_cancel_put) | **PUT** /v1/organizations/consent-requests/{request_id}/cancel | Cancel the  Organization data request by id.
[**v1_organizations_consent_requests_request_id_get**](OrganizationsApi.md#v1_organizations_consent_requests_request_id_get) | **GET** /v1/organizations/consent-requests/{request_id} | Get Organization data request by id.
[**v1_organizations_consent_templates_get**](OrganizationsApi.md#v1_organizations_consent_templates_get) | **GET** /v1/organizations/consent-templates | Get the paginated list of organization consent templates.
[**v1_organizations_consent_templates_template_id_get**](OrganizationsApi.md#v1_organizations_consent_templates_template_id_get) | **GET** /v1/organizations/consent-templates/{template_id} | Get Organization consent template details by consent id.
[**v1_organizations_consents_consent_id_documents_document_id_download_get**](OrganizationsApi.md#v1_organizations_consents_consent_id_documents_document_id_download_get) | **GET** /v1/organizations/consents/{consent_id}/documents/{document_id}/download | Download Organization consent document by document id.
[**v1_organizations_consents_consent_id_documents_document_id_get**](OrganizationsApi.md#v1_organizations_consents_consent_id_documents_document_id_get) | **GET** /v1/organizations/consents/{consent_id}/documents/{document_id} | Get Organization consent document based on document id and consent id.
[**v1_organizations_consents_consent_id_documents_get**](OrganizationsApi.md#v1_organizations_consents_consent_id_documents_get) | **GET** /v1/organizations/consents/{consent_id}/documents | Get Organization consent document by consent id.
[**v1_organizations_consents_consent_id_financial_accounts_account_id_get**](OrganizationsApi.md#v1_organizations_consents_consent_id_financial_accounts_account_id_get) | **GET** /v1/organizations/consents/{consent_id}/financial-accounts/{account_id} | Get organization consented financial account details.
[**v1_organizations_consents_consent_id_financial_accounts_account_id_transactions_get**](OrganizationsApi.md#v1_organizations_consents_consent_id_financial_accounts_account_id_transactions_get) | **GET** /v1/organizations/consents/{consent_id}/financial-accounts/{account_id}/transactions | Get organization consented financial account transactions.
[**v1_organizations_consents_consent_id_financial_accounts_get**](OrganizationsApi.md#v1_organizations_consents_consent_id_financial_accounts_get) | **GET** /v1/organizations/consents/{consent_id}/financial-accounts | Get all organization consented financial accounts.
[**v1_organizations_consents_consent_id_get**](OrganizationsApi.md#v1_organizations_consents_consent_id_get) | **GET** /v1/organizations/consents/{consent_id} | Get Organization consent request details by consent id.
[**v1_organizations_consents_get**](OrganizationsApi.md#v1_organizations_consents_get) | **GET** /v1/organizations/consents | Get the paginated list of Organization consents.
[**v1_organizations_documents_issue_post**](OrganizationsApi.md#v1_organizations_documents_issue_post) | **POST** /v1/organizations/documents/issue | Issue a new document to an organization.
[**v1_organizations_documents_issue_upload_issue_request_id_post**](OrganizationsApi.md#v1_organizations_documents_issue_upload_issue_request_id_post) | **POST** /v1/organizations/documents/issue/upload/{issue_request_id} | Upload a document for issuance request of Organization.
[**v1_organizations_documents_issued_document_id_get**](OrganizationsApi.md#v1_organizations_documents_issued_document_id_get) | **GET** /v1/organizations/documents/issued/{document_id} | Get issued document.
[**v1_organizations_documents_issued_get**](OrganizationsApi.md#v1_organizations_documents_issued_get) | **GET** /v1/organizations/documents/issued | Get paginated list of issued documents of given document type.
[**v1_organizations_documents_types_get**](OrganizationsApi.md#v1_organizations_documents_types_get) | **GET** /v1/organizations/documents/types | Get paginated list of registered document types.



## v1_organizations_consent_requests_get

> crate::models::PaginatedListOfOrganizationConsentRequestDetailss v1_organizations_consent_requests_get(status, from_date_time, to_date_time, page_no, page_size)
Get all consent requests sent to Organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<[**DataConsentStatus**](.md)> |  |  |
**from_date_time** | Option<**String**> |  |  |
**to_date_time** | Option<**String**> |  |  |
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfOrganizationConsentRequestDetailss**](PaginatedListOfOrganizationConsentRequestDetailss.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_organizations_consent_requests_post

> crate::models::OrganizationConsentRequestDetails v1_organizations_consent_requests_post(create_consent_request)
Create consent request for an Organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_consent_request** | [**CreateConsentRequest**](CreateConsentRequest.md) |  | [required] |

### Return type

[**crate::models::OrganizationConsentRequestDetails**](OrganizationConsentRequestDetails.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_organizations_consent_requests_request_id_cancel_put

> bool v1_organizations_consent_requests_request_id_cancel_put(request_id)
Cancel the  Organization data request by id.

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


## v1_organizations_consent_requests_request_id_get

> crate::models::ConsentRequest v1_organizations_consent_requests_request_id_get(request_id)
Get Organization data request by id.

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


## v1_organizations_consent_templates_get

> crate::models::PaginatedListOfConsentRequestTemplates v1_organizations_consent_templates_get(page_no, page_size)
Get the paginated list of organization consent templates.

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


## v1_organizations_consent_templates_template_id_get

> crate::models::OrganizationConsentRequestTemplateDetails v1_organizations_consent_templates_template_id_get(template_id)
Get Organization consent template details by consent id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |

### Return type

[**crate::models::OrganizationConsentRequestTemplateDetails**](OrganizationConsentRequestTemplateDetails.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_organizations_consents_consent_id_documents_document_id_download_get

> std::path::PathBuf v1_organizations_consents_consent_id_documents_document_id_download_get(consent_id, document_id)
Download Organization consent document by document id.

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


## v1_organizations_consents_consent_id_documents_document_id_get

> crate::models::ConsentedDocument v1_organizations_consents_consent_id_documents_document_id_get(consent_id, document_id)
Get Organization consent document based on document id and consent id.

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


## v1_organizations_consents_consent_id_documents_get

> Vec<crate::models::ConsentedDocument> v1_organizations_consents_consent_id_documents_get(consent_id)
Get Organization consent document by consent id.

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


## v1_organizations_consents_consent_id_financial_accounts_account_id_get

> crate::models::FinancialAccount v1_organizations_consents_consent_id_financial_accounts_account_id_get(consent_id, account_id)
Get organization consented financial account details.

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


## v1_organizations_consents_consent_id_financial_accounts_account_id_transactions_get

> crate::models::PaginatedListOfFinancialAccountTransactions v1_organizations_consents_consent_id_financial_accounts_account_id_transactions_get(consent_id, account_id, from_date_time, to_date_time, page_no, page_size)
Get organization consented financial account transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**consent_id** | **String** |  | [required] |
**account_id** | **String** |  | [required] |
**from_date_time** | Option<**String**> |  |  |
**to_date_time** | Option<**String**> |  |  |
**page_no** | Option<**i32**> |  |  |
**page_size** | Option<**i32**> |  |  |

### Return type

[**crate::models::PaginatedListOfFinancialAccountTransactions**](PaginatedListOfFinancialAccountTransactions.md)

### Authorization

[OAuth2ClientCredentials](../README.md#OAuth2ClientCredentials)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v1_organizations_consents_consent_id_financial_accounts_get

> Vec<crate::models::FinancialAccount> v1_organizations_consents_consent_id_financial_accounts_get(consent_id)
Get all organization consented financial accounts.

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


## v1_organizations_consents_consent_id_get

> crate::models::ConsentDetails v1_organizations_consents_consent_id_get(consent_id)
Get Organization consent request details by consent id.

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


## v1_organizations_consents_get

> crate::models::PaginatedListOfConsents v1_organizations_consents_get(status, from_date_time, to_date_time, page_no, page_size)
Get the paginated list of Organization consents.

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


## v1_organizations_documents_issue_post

> crate::models::DocumentIssueRequestDetails v1_organizations_documents_issue_post(document_issue_request)
Issue a new document to an organization.

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


## v1_organizations_documents_issue_upload_issue_request_id_post

> crate::models::UploadDocumentResponse v1_organizations_documents_issue_upload_issue_request_id_post(issue_request_id, file)
Upload a document for issuance request of Organization.

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


## v1_organizations_documents_issued_document_id_get

> crate::models::IssuedDocument v1_organizations_documents_issued_document_id_get(document_id)
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


## v1_organizations_documents_issued_get

> crate::models::PaginatedListOfIssuedDocuments v1_organizations_documents_issued_get(document_type_id, from_date_time, to_date_time, page_no, page_size)
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


## v1_organizations_documents_types_get

> crate::models::PaginatedListOfDocumentTypes v1_organizations_documents_types_get(page_no, page_size)
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

