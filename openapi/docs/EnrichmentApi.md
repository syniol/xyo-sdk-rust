# \EnrichmentApi

All URIs are relative to *https://api.xyo.financial*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enrich_transaction**](EnrichmentApi.md#enrich_transaction) | **POST** /v1/ai/finance/enrichment/transaction | Transaction Enrichment
[**enrich_transactions**](EnrichmentApi.md#enrich_transactions) | **POST** /v1/ai/finance/enrichment/transactions | Transaction Enrichments
[**get_enrichment_status**](EnrichmentApi.md#get_enrichment_status) | **GET** /v1/ai/finance/enrichment/status/{id} | Transaction Enrichments Status



## enrich_transaction

> models::EnrichmentResponse enrich_transaction(enrichment_request)
Transaction Enrichment

Enrich a single financial transaction synchronously.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enrichment_request** | Option<[**EnrichmentRequest**](EnrichmentRequest.md)> |  |  |

### Return type

[**models::EnrichmentResponse**](EnrichmentResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrich_transactions

> models::EnrichTransactionCollectionResponse enrich_transactions(x_api_user, enrich_transactions_request_inner)
Transaction Enrichments

Enrich a collection of financial transactions asynchronously.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_api_user** | Option<[**serde_json::Value**](SerdeJson__Value.md)> |  |  |
**enrich_transactions_request_inner** | Option<[**Vec<models::EnrichTransactionsRequestInner>**](EnrichTransactionsRequestInner.md)> |  |  |

### Return type

[**models::EnrichTransactionCollectionResponse**](EnrichTransactionCollectionResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enrichment_status

> models::EnrichmentCollectionStatusResponse get_enrichment_status(id, x_api_user)
Transaction Enrichments Status

Get the status of an asynchronous bulk enrichment job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**x_api_user** | Option<**String**> |  |  |

### Return type

[**models::EnrichmentCollectionStatusResponse**](EnrichmentCollectionStatusResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

