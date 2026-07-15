# UpdateDocumentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table** | **String** | Name of the document table | 
**cluster** | Option<**String**> | Name of the document cluster | [optional]
**doc** | [**serde_json::Value**](.md) | Object containing the document fields to update | 
**id** | Option<**i32**> | Document ID | [optional]
**query** | Option<[**models::QueryFilter**](queryFilter.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


