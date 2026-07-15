# HitsHits

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | Option<**i32**> | The ID of the matched document | [optional]
**_score** | Option<**i32**> | The score of the matched document | [optional]
**_source** | Option<[**serde_json::Value**](.md)> | The source data of the matched document | [optional]
**_knn_dist** | Option<**f64**> | The knn distance of the matched document returned for knn queries | [optional]
**highlight** | Option<[**serde_json::Value**](.md)> | The highlighting-related data of the matched document | [optional]
**table** | Option<**String**> | The table name of the matched document returned for percolate queries | [optional]
**_type_colon** | Option<**String**> | The type of the matched document returned for percolate queries | [optional]
**fields** | Option<[**serde_json::Value**](.md)> | The percolate-related fields of the matched document returned for percolate queries | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


