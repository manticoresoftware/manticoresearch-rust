# Chat

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | **String** | User question to send to the chat model | 
**table** | **String** | Vectorized table to retrieve context from | 
**model_name** | **String** | Name of the chat model | 
**conversation_uuid** | Option<**String**> | Existing conversation id to continue the dialog, or an empty string to start a new conversation. If omitted, a new id is generated.  | [optional]
**vector_field** | Option<**String**> | A specific vector field to search by. If omitted, Buddy uses the first `FLOAT_VECTOR` field from `SHOW CREATE TABLE`.  | [optional]
**fields** | Option<**String**> | Legacy alias for `vector_field`. A request must not include both `vector_field` and `fields`.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


