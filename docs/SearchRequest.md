# SearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table** | Option<**String**> | The table to perform the search on | [optional]
**chat** | Option<[**models::Chat**](chat.md)> |  | [optional]
**query** | Option<[**models::SearchQuery**](searchQuery.md)> |  | [optional]
**join** | Option<[**Vec<models::Join>**](join.md)> | Join clause to combine search data from multiple tables | [optional]
**highlight** | Option<[**models::Highlight**](highlight.md)> |  | [optional]
**limit** | Option<**i32**> | Maximum number of results to return | [optional]
**knn** | Option<[**models::Knn**](knn.md)> | K-nearest neighbor search settings. Pass a single `knn` object or an array of objects for multi-vector search.  | [optional]
**hybrid** | Option<[**models::Hybrid**](hybrid.md)> |  | [optional]
**facet_filter_mode** | Option<[**models::FacetFilterMode**](facetFilterMode.md)> |  | [optional]
**aggs** | Option<[**std::collections::HashMap<String, models::Aggregation>**](aggregation.md)> | Defines aggregation settings for grouping results | [optional]
**expressions** | Option<**std::collections::HashMap<String, String>**> | Expressions to calculate additional values for the result. Simpler alternative to `script_fields`; expression names must be lowercase.  | [optional]
**script_fields** | Option<[**std::collections::HashMap<String, models::ScriptField>**](scriptField.md)> | Named expressions computed at search time. Each value defines an inline script whose result is stored under the field name. For more information see [Expressions](https://manual.manticoresearch.com/Searching/Expressions#script_fields)  | [optional]
**max_matches** | Option<**i32**> | Maximum number of matches allowed in the result | [optional]
**offset** | Option<**i32**> | Starting point for pagination of the result | [optional]
**options** | Option<[**serde_json::Value**](.md)> | Additional search options | [optional]
**profile** | Option<**bool**> | Enable or disable profiling of the search request | [optional]
**sort** | Option<[**serde_json::Value**](.md)> |  | [optional]
**_source** | Option<[**serde_json::Value**](.md)> |  | [optional]
**track_scores** | Option<**bool**> | Enable or disable result weight calculation used for sorting | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


