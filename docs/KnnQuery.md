# KnnQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | Field to perform the k-nearest neighbor search on | 
**k** | **i32** | The number of nearest neighbors to return | 
**query_vector** | Option<**Vec<f32>**> | The vector used as input for the KNN search | [optional]
**doc_id** | Option<**i64**> | The docuemnt ID used as input for the KNN search | [optional]
**ef** | Option<**i32**> | Optional parameter controlling the accuracy of the search | [optional]
**filter** | Option<[**crate::models::QueryFilter**](QueryFilter.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


