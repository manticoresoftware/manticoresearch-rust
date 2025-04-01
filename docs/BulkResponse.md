# BulkResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**items** | Option<[**Vec<serde_json::Value>**](Serde_json::Value.md)> | List of results | [optional]
**errors** | Option<**bool**> | Errors occurred during the bulk operation | [optional]
**error** | Option<**String**> | Error message describing an error if such occurred | [optional]
**current_line** | Option<**i32**> | Number of the row returned in the response | [optional]
**skipped_lines** | Option<**i32**> | Number of rows skipped in the response | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


