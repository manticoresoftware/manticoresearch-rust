# \UtilsApi

All URIs are relative to *http://127.0.0.1:9308*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sql**](UtilsApi.md#sql) | **Post** /sql | Perform SQL requests
[**token**](UtilsApi.md#token) | **Post** /token | Create or rotate a bearer token



## sql

> models::SqlResponse sql(body, raw_response)
Perform SQL requests

Run a query in SQL format. Expects a query string passed through `body` parameter and optional `raw_response` parameter that defines a format of response. `raw_response` can be set to `False` for Select queries only, e.g., `SELECT * FROM mytable` The query string must stay as it is, no URL encoding is needed. The response object depends on the query executed. In select mode the response has same format as `/search` operation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **String** | A query parameter string.  | [required] |
**raw_response** | Option<**bool**> | Optional parameter, defines a format of response. Can be set to `False` for Select only queries and set to `True` for any type of queries. Default value is 'True'.  |  |[default to true]

### Return type

[**models::SqlResponse**](sqlResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token

> String token(body)
Create or rotate a bearer token

Create or rotate a bearer token for the authenticated HTTP user. Requires HTTP Basic authentication with a Manticore user name and password. The endpoint returns the raw token. Example:    ```   curl -u admin:password -X POST http://127.0.0.1:9308/token -d \"{}\"   ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |

### Return type

**String**

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

