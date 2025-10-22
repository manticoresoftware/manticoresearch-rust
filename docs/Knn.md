# Knn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | Field to perform the k-nearest neighbor search on | 
**k** | **i32** | The number of nearest neighbors to return | 
**query** | Option<[**models::KnnQuery**](knn_query.md)> |  | [optional]
**query_vector** | Option<**Vec<f64>**> | The vector used as input for the KNN search | [optional]
**doc_id** | Option<**i32**> | The docuemnt ID used as input for the KNN search | [optional]
**ef** | Option<**i32**> | Optional parameter controlling the accuracy of the search | [optional]
**filter** | Option<[**models::QueryFilter**](queryFilter.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


