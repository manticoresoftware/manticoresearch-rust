# AggPercentiles

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | Numeric field to calculate percentiles for | 
**values** | Option<**Vec<f64>**> | Percentile points to compute (0-100). Defaults to 1, 5, 25, 50, 75, 95, and 99 when omitted.  | [optional]
**keyed** | Option<**bool**> | Return an object keyed by percentile when true, or an array when false. Default is false.  | [optional]
**tdigest** | Option<[**models::AggTDigest**](aggTDigest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


