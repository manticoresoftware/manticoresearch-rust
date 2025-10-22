# InsertDocumentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table** | **String** | Name of the table to insert the document into | 
**cluster** | Option<**String**> | Name of the cluster to insert the document into | [optional]
**id** | Option<**i32**> | Document ID. If not provided, an ID will be auto-generated  | [optional]
**doc** | [**serde_json::Value**](.md) | Object containing document data  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


