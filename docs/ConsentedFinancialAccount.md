# ConsentedFinancialAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Financial account id. | 
**name** | **String** | Financial account name. | 
**category** | [**crate::models::FinancialAccountCategoryType**](FinancialAccountCategoryType.md) |  | 
**sub_category** | [**crate::models::FinancialAccountSubCategoryType**](FinancialAccountSubCategoryType.md) |  | 
**identifier** | **String** | Financial account identifier. | 
**field_title** | **String** | Financial account field title. | 
**field_slug** | **String** | Financial account field slug. | 
**requested_details** | [**Vec<crate::models::FinancialAccountDetailsRequired>**](FinancialAccountDetailsRequired.md) | Requested financial account details. | 
**transaction_period** | Option<[**crate::models::ConsentedFinancialAccountTransactionPeriod**](ConsentedFinancialAccount_transactionPeriod.md)> |  | [optional]
**issuer_id** | **String** | Financial account issuer id. | 
**issuer_name** | **String** | Financial account issuer name. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


