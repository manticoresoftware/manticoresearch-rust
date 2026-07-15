# AggPercentileRanks

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | Numeric field to calculate percentile ranks for | 
**values** | **Vec<f64>** | Input values to rank | 
**keyed** | Option<**bool**> | Return an object keyed by input value when true, or an array when false. Default is false.  | [optional]
**tdigest** | Option<[**models::AggTDigest**](aggTDigest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


