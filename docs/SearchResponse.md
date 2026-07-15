# SearchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**took** | Option<**i32**> | Time taken to execute the search | [optional]
**timed_out** | Option<**bool**> | Indicates whether the search operation timed out | [optional]
**aggregations** | Option<[**std::collections::HashMap<String, models::AggBucketsResult>**](aggBucketsResult.md)> | Aggregated search results grouped by the specified criteria. Each named aggregation typically contains a `buckets` array (or keyed map) of bucket objects with `key`, `doc_count`, and optional `status`.  | [optional]
**hits** | Option<[**models::SearchResponseHits**](searchResponse_hits.md)> |  | [optional]
**profile** | Option<[**serde_json::Value**](.md)> | Profile information about the search execution, if profiling is enabled | [optional]
**scroll** | Option<**String**> | Scroll token to be used fo pagination | [optional]
**warning** | Option<[**serde_json::Value**](.md)> | Warnings encountered during the search operation | [optional]
**conversation_uuid** | Option<**String**> | Existing or generated conversation id (conversational search) | [optional]
**user_query** | Option<**String**> | Original user query (conversational search) | [optional]
**search_query** | Option<**String**> | Standalone search query used for KNN retrieval (conversational search) | [optional]
**response** | Option<**String**> | LLM answer as generated (conversational search) | [optional]
**sources** | Option<**String**> | JSON string containing retrieved source rows used as LLM context (conversational search).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


