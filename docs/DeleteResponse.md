# DeleteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table** | Option<**String**> | The name of the table from which the document was deleted | [optional]
**deleted** | Option<**i32**> | Number of documents deleted | [optional]
**id** | Option<**i32**> | The ID of the deleted document. If multiple documents are deleted, the ID of the first deleted document is returned | [optional]
**found** | Option<**bool**> | Indicates whether any documents to be deleted were found | [optional]
**result** | Option<**String**> | Result of the delete operation, typically 'deleted' | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


