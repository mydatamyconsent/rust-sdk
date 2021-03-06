# Rust API client for mydatamyconsent

Unleashing the power of data consent by establishing trust. The Platform Core Developer API defines a set of capabilities that can be used to request, issue, manage and update data, documents and credentials by organizations. The API can be used to request, manage and update Decentralised Identifiers, Financial Data, Health Data issue Documents, Credentials directly or using OpenID Connect flows, and verify Messages signed with DIDs and much more.

For more information, please visit [https://mydatamyconsent.com/en-us/developers](https://mydatamyconsent.com/en-us/developers)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: 0.0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `mydatamyconsent` and add the following to `Cargo.toml` under `[dependencies]`:

```
mydatamyconsent = { path = "./mydatamyconsent" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.mydatamyconsent.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DataConsentRequestsApi* | [**cancel_individual_data_consent_request**](docs/DataConsentRequestsApi.md#cancel_individual_data_consent_request) | **PUT** /v1/consent-requests/individual/{requestId}/cancel | Cancel the individual data consent request by Id.
*DataConsentRequestsApi* | [**cancel_organization_data_consent_request**](docs/DataConsentRequestsApi.md#cancel_organization_data_consent_request) | **PUT** /v1/consent-requests/organization/{requestId}/cancel | Cancel the organization data consent request by Id.
*DataConsentRequestsApi* | [**create_individual_data_consent_request**](docs/DataConsentRequestsApi.md#create_individual_data_consent_request) | **POST** /v1/consent-requests/individual | Create data consent request for an individual.
*DataConsentRequestsApi* | [**create_organization_data_consent_request**](docs/DataConsentRequestsApi.md#create_organization_data_consent_request) | **POST** /v1/consent-requests/organization | Create data consent request for an organization.
*DataConsentRequestsApi* | [**get_all_consent_requests_to_individuals**](docs/DataConsentRequestsApi.md#get_all_consent_requests_to_individuals) | **GET** /v1/consent-requests/individuals | Get all Consent Requests sent to individuals.
*DataConsentRequestsApi* | [**get_all_consent_requests_to_organizations**](docs/DataConsentRequestsApi.md#get_all_consent_requests_to_organizations) | **GET** /v1/consent-requests/organizations | Get all Consent Requests sent to organizations.
*DataConsentRequestsApi* | [**get_individual_consent_request_by_id**](docs/DataConsentRequestsApi.md#get_individual_consent_request_by_id) | **GET** /v1/consent-requests/individuals/{requestId} | Get individual data consent request by id.
*DataConsentRequestsApi* | [**get_organization_consent_request_by_id**](docs/DataConsentRequestsApi.md#get_organization_consent_request_by_id) | **GET** /v1/consent-requests/organizations/{requestId} | Get a OrganizationConsent Request by Id.
*DataConsentsApi* | [**download_consented_document_analysis**](docs/DataConsentsApi.md#download_consented_document_analysis) | **GET** /v1/consents/{consentId}/documents/{documentId}/analysis | Get analysis of a consented document.
*DataConsentsApi* | [**download_individual_consented_document_by_id**](docs/DataConsentsApi.md#download_individual_consented_document_by_id) | **GET** /v1/consents/individuals/{consentId}/documents/{documentId}/download | Download individual consented document by document id.
*DataConsentsApi* | [**download_organization_consented_document_by_id**](docs/DataConsentsApi.md#download_organization_consented_document_by_id) | **GET** /v1/consents/organizations/{consentId}/documents/{documentId}/download | Download organization consent document based on document id.
*DataConsentsApi* | [**get_all_consented_financial_accounts**](docs/DataConsentsApi.md#get_all_consented_financial_accounts) | **GET** /v1/consents/individuals/{consentId}/financial-accounts | Get all individual consented financial accounts.
*DataConsentsApi* | [**get_consent_financial_accounts**](docs/DataConsentsApi.md#get_consent_financial_accounts) | **GET** /v1/consents/organizations/{consentId}/financial-accounts | Get all organizational consented financial accounts.
*DataConsentsApi* | [**get_consented_account_by_id**](docs/DataConsentsApi.md#get_consented_account_by_id) | **GET** /v1/consents/individuals/{consentId}/financial-accounts/{accountId} | Get individual consented financial account details based on account id.
*DataConsentsApi* | [**get_consented_document_by_id**](docs/DataConsentsApi.md#get_consented_document_by_id) | **GET** /v1/consents/individuals/{consentId}/documents/{documentId} | Get individual consented document by document id.
*DataConsentsApi* | [**get_consented_financial_account**](docs/DataConsentsApi.md#get_consented_financial_account) | **GET** /v1/consents/organizations/{consentId}/financial-accounts/{accountId} | Get organization consented financial account details based on account id.
*DataConsentsApi* | [**get_consented_financial_account_insights**](docs/DataConsentsApi.md#get_consented_financial_account_insights) | **GET** /v1/consents/{consentId}/financial-accounts/{accountId}/insights | Get consented financial account insights.
*DataConsentsApi* | [**get_consented_financial_account_transactions**](docs/DataConsentsApi.md#get_consented_financial_account_transactions) | **GET** /v1/consents/individuals/{consentId}/financial-accounts/{accountId}/transactions | Get individual consented financial account transactions of an individual based on accountId.
*DataConsentsApi* | [**get_consents**](docs/DataConsentsApi.md#get_consents) | **GET** /v1/consents/individuals | Get the paginated list of individual data consents.
*DataConsentsApi* | [**get_individual_consented_documents**](docs/DataConsentsApi.md#get_individual_consented_documents) | **GET** /v1/consents/individuals/{consentId}/documents | Get individual consented documents by consent id.
*DataConsentsApi* | [**get_individual_data_consent_by_id**](docs/DataConsentsApi.md#get_individual_data_consent_by_id) | **GET** /v1/consents/individuals/{consentId} | Get individuals data consent details by consent id.
*DataConsentsApi* | [**get_org_consented_account_transactions**](docs/DataConsentsApi.md#get_org_consented_account_transactions) | **GET** /v1/consents/organizations/{consentId}/financial-accounts/{accountId}/transactions | Get organization consented financial account transactions of an individual based on accountId.
*DataConsentsApi* | [**get_organization_consented_document_by_id**](docs/DataConsentsApi.md#get_organization_consented_document_by_id) | **GET** /v1/consents/organizations/{consentId}/documents/{documentId} | Get organization consent document based on document id.
*DataConsentsApi* | [**get_organization_consented_documents**](docs/DataConsentsApi.md#get_organization_consented_documents) | **GET** /v1/consents/organizations/{consentId}/documents | Get organization consented documents by consent id.
*DataConsentsApi* | [**get_organization_data_consent_by_id**](docs/DataConsentsApi.md#get_organization_data_consent_by_id) | **GET** /v1/consents/organizations/{consentId} | Get organizations data consent details by consent id.
*DataConsentsApi* | [**get_organization_data_consents**](docs/DataConsentsApi.md#get_organization_data_consents) | **GET** /v1/consents/organizations | Get the paginated list of organization data consents.
*DataProcessingAgreementsApi* | [**create_data_processing_agreement**](docs/DataProcessingAgreementsApi.md#create_data_processing_agreement) | **POST** /v1/data-agreements | Create a data processing agreement.
*DataProcessingAgreementsApi* | [**delete_data_processing_agreement_by_id**](docs/DataProcessingAgreementsApi.md#delete_data_processing_agreement_by_id) | **DELETE** /v1/data-agreements/{id} | Delete a data processing agreement. This will not delete a published or a agreement in use with consents.
*DataProcessingAgreementsApi* | [**get_data_processing_agreement_by_id**](docs/DataProcessingAgreementsApi.md#get_data_processing_agreement_by_id) | **GET** /v1/data-agreements/{id} | Get data processing agreement by id.
*DataProcessingAgreementsApi* | [**get_data_processing_agreements**](docs/DataProcessingAgreementsApi.md#get_data_processing_agreements) | **GET** /v1/data-agreements | Get paginated data processing agreements.
*DataProcessingAgreementsApi* | [**terminate_data_processing_agreement_by_id**](docs/DataProcessingAgreementsApi.md#terminate_data_processing_agreement_by_id) | **PUT** /v1/data-agreements/{id}/terminate | Terminate a data processing agreement.
*DataProcessingAgreementsApi* | [**update_data_processing_agreement**](docs/DataProcessingAgreementsApi.md#update_data_processing_agreement) | **PUT** /v1/data-agreements/{id} | Update a data processing agreement.
*DataProviderDiscoveryApi* | [**get_data_provider_by_id**](docs/DataProviderDiscoveryApi.md#get_data_provider_by_id) | **GET** /v1/data-providers/{providerId} | Get a Data Provider details by provider id.
*DataProviderDiscoveryApi* | [**get_data_providers**](docs/DataProviderDiscoveryApi.md#get_data_providers) | **GET** /v1/data-providers | Discover all data providers in My Data My Consent by country and filters.
*DigiLockerCompatIssuerApi* | [**digilocker_compat_issue_document**](docs/DigiLockerCompatIssuerApi.md#digilocker_compat_issue_document) | **POST** /issuer/issuedoc/1/xml | Digilocker Compatible endpoint to issue document.
*DocumentsApi* | [**get_issued_document_by_id**](docs/DocumentsApi.md#get_issued_document_by_id) | **GET** /v1/documents/issued/{documentId} | Get issued document.
*DocumentsApi* | [**get_issued_documents**](docs/DocumentsApi.md#get_issued_documents) | **GET** /v1/documents/issued | Get paginated list of issued documents of given document type.
*DocumentsApi* | [**get_registered_document_types**](docs/DocumentsApi.md#get_registered_document_types) | **GET** /v1/documents/types | Get paginated list of registered document types.
*DocumentsApi* | [**issue_document_to_individual**](docs/DocumentsApi.md#issue_document_to_individual) | **POST** /v1/documents/issue/individual | Issue a new document to an individual user.
*DocumentsApi* | [**issue_document_to_organization**](docs/DocumentsApi.md#issue_document_to_organization) | **POST** /v1/documents/issue/organization | Issue a new document to an organization.
*DocumentsApi* | [**upload_document_for_individual**](docs/DocumentsApi.md#upload_document_for_individual) | **POST** /v1/documents/issue/individual/upload/{issueRequestId} | Upload a document for issuance request of individual.
*DocumentsApi* | [**upload_document_for_organization**](docs/DocumentsApi.md#upload_document_for_organization) | **POST** /v1/documents/issue/organization/upload/{issueRequestId} | Upload a document for issuance request of organization.
*SupportedIdentifiersApi* | [**get_all_supported_identifiers**](docs/SupportedIdentifiersApi.md#get_all_supported_identifiers) | **GET** /v1/supported-identifiers/{countryIso2Code} | Get all supported identifiers by country.


## Documentation For Models

 - [Activity](docs/Activity.md)
 - [ApprovedConsentRequest](docs/ApprovedConsentRequest.md)
 - [BankAccountType](docs/BankAccountType.md)
 - [BillPaymentOrderItem](docs/BillPaymentOrderItem.md)
 - [CollectibleTypes](docs/CollectibleTypes.md)
 - [ConsentRequestReceiver](docs/ConsentRequestReceiver.md)
 - [CreateDataConsentRequest](docs/CreateDataConsentRequest.md)
 - [CreateDataProcessingAgreement](docs/CreateDataProcessingAgreement.md)
 - [DataConsent](docs/DataConsent.md)
 - [DataConsentDetails](docs/DataConsentDetails.md)
 - [DataConsentDetailsPaginatedList](docs/DataConsentDetailsPaginatedList.md)
 - [DataConsentDocument](docs/DataConsentDocument.md)
 - [DataConsentDocumentIssuer](docs/DataConsentDocumentIssuer.md)
 - [DataConsentFinancialsDto](docs/DataConsentFinancialsDto.md)
 - [DataConsentRequest](docs/DataConsentRequest.md)
 - [DataConsentRequestDetails](docs/DataConsentRequestDetails.md)
 - [DataConsentRequestedFinancialAccount](docs/DataConsentRequestedFinancialAccount.md)
 - [DataConsentStatus](docs/DataConsentStatus.md)
 - [DataProcessingAgreement](docs/DataProcessingAgreement.md)
 - [DataProcessingAgreementBase](docs/DataProcessingAgreementBase.md)
 - [DataProcessingAgreementPaginatedList](docs/DataProcessingAgreementPaginatedList.md)
 - [DataProtectionOfficer](docs/DataProtectionOfficer.md)
 - [DataProvider](docs/DataProvider.md)
 - [DataProviderPaginatedList](docs/DataProviderPaginatedList.md)
 - [DocumentCategoryType](docs/DocumentCategoryType.md)
 - [DocumentDigitalSignature](docs/DocumentDigitalSignature.md)
 - [DocumentIssueRequest](docs/DocumentIssueRequest.md)
 - [DocumentIssueRequestDetails](docs/DocumentIssueRequestDetails.md)
 - [DocumentIssueRequestStatus](docs/DocumentIssueRequestStatus.md)
 - [DocumentReceiver](docs/DocumentReceiver.md)
 - [DocumentSubCategoryType](docs/DocumentSubCategoryType.md)
 - [DocumentType](docs/DocumentType.md)
 - [DocumentTypePaginatedList](docs/DocumentTypePaginatedList.md)
 - [DocumentsRequired](docs/DocumentsRequired.md)
 - [Error](docs/Error.md)
 - [ErrorType](docs/ErrorType.md)
 - [Financial](docs/Financial.md)
 - [FinancialAccount](docs/FinancialAccount.md)
 - [FinancialAccountDetailsRequired](docs/FinancialAccountDetailsRequired.md)
 - [FinancialAccounts](docs/FinancialAccounts.md)
 - [IdentificationStrategy](docs/IdentificationStrategy.md)
 - [Identifier](docs/Identifier.md)
 - [IndividualDataConsentRequestDetails](docs/IndividualDataConsentRequestDetails.md)
 - [IndividualDataConsentRequestDetailsPaginatedList](docs/IndividualDataConsentRequestDetailsPaginatedList.md)
 - [IssuedDocument](docs/IssuedDocument.md)
 - [IssuedDocumentDetails](docs/IssuedDocumentDetails.md)
 - [IssuedDocumentPaginatedList](docs/IssuedDocumentPaginatedList.md)
 - [Life](docs/Life.md)
 - [OrganizationDataConsentRequestDetails](docs/OrganizationDataConsentRequestDetails.md)
 - [OrganizationDataConsentRequestDetailsPaginatedList](docs/OrganizationDataConsentRequestDetailsPaginatedList.md)
 - [OrganizationFinancialAccountDto](docs/OrganizationFinancialAccountDto.md)
 - [OrganizationFinancialTransactionsDto](docs/OrganizationFinancialTransactionsDto.md)
 - [OrganizationFinancialTransactionsDtoPaginatedList](docs/OrganizationFinancialTransactionsDtoPaginatedList.md)
 - [PaymentRequest](docs/PaymentRequest.md)
 - [ProblemDetails](docs/ProblemDetails.md)
 - [PushUriRequest](docs/PushUriRequest.md)
 - [PushUriResponse](docs/PushUriResponse.md)
 - [SharedWith](docs/SharedWith.md)
 - [StringStringKeyValuePair](docs/StringStringKeyValuePair.md)
 - [SupportedEntityType](docs/SupportedEntityType.md)
 - [SupportedIdentifier](docs/SupportedIdentifier.md)
 - [UpdateDataProcessingAgreement](docs/UpdateDataProcessingAgreement.md)
 - [UriDetails](docs/UriDetails.md)
 - [UserAccountFinancialTransactionsDto](docs/UserAccountFinancialTransactionsDto.md)
 - [UserAccountFinancialTransactionsDtoPaginatedList](docs/UserAccountFinancialTransactionsDtoPaginatedList.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@mydatamyconsent.com

