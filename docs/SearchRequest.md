# SearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table** | **String** | The table to perform the search on | 
**query** | Option<[**models::SearchQuery**](searchQuery.md)> |  | [optional]
**join** | Option<[**Vec<models::Join>**](join.md)> | Join clause to combine search data from multiple tables | [optional]
**highlight** | Option<[**models::Highlight**](highlight.md)> |  | [optional]
**limit** | Option<**i32**> | Maximum number of results to return | [optional]
**knn** | Option<[**models::Knn**](knn.md)> |  | [optional]
**aggs** | Option<[**std::collections::HashMap<String, models::Aggregation>**](aggregation.md)> | Defines aggregation settings for grouping results | [optional]
**expressions** | Option<**std::collections::HashMap<String, String>**> | Expressions to calculate additional values for the result | [optional]
**max_matches** | Option<**i32**> | Maximum number of matches allowed in the result | [optional]
**offset** | Option<**i32**> | Starting point for pagination of the result | [optional]
**options** | Option<[**serde_json::Value**](.md)> | Additional search options | [optional]
**profile** | Option<**bool**> | Enable or disable profiling of the search request | [optional]
**sort** | Option<[**serde_json::Value**](.md)> |  | [optional]
**_source** | Option<[**serde_json::Value**](.md)> |  | [optional]
**track_scores** | Option<**bool**> | Enable or disable result weight calculation used for sorting | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


