# Highlight

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fragment_size** | Option<[**serde_json::Value**](.md)> | Maximum size of the text fragments in highlighted snippets per field | [optional]
**limit** | Option<[**serde_json::Value**](.md)> | Maximum size of snippets per field | [optional]
**limit_snippets** | Option<[**serde_json::Value**](.md)> | Maximum number of snippets per field | [optional]
**limit_words** | Option<[**serde_json::Value**](.md)> | Maximum number of words per field | [optional]
**number_of_fragments** | Option<[**serde_json::Value**](.md)> | Total number of highlighted fragments per field | [optional]
**after_match** | Option<**String**> | Text inserted after the matched term, typically used for HTML formatting | [optional][default to </strong>]
**allow_empty** | Option<**bool**> | Permits an empty string to be returned as the highlighting result. Otherwise, the beginning of the original text would be returned | [optional]
**around** | Option<**i32**> | Number of words around the match to include in the highlight | [optional]
**before_match** | Option<**String**> | Text inserted before the match, typically used for HTML formatting | [optional][default to <strong>]
**emit_zones** | Option<**bool**> | Emits an HTML tag with the enclosing zone name before each highlighted snippet | [optional]
**encoder** | Option<**String**> | If set to 'html', retains HTML markup when highlighting | [optional]
**fields** | Option<[**serde_json::Value**](.md)> |  | [optional]
**force_all_words** | Option<**bool**> | Ignores the length limit until the result includes all keywords | [optional]
**force_snippets** | Option<**bool**> | Forces snippet generation even if limits allow highlighting the entire text | [optional]
**highlight_query** | Option<[**crate::models::QueryFilter**](queryFilter.md)> |  | [optional]
**html_strip_mode** | Option<**String**> | Defines the mode for handling HTML markup in the highlight | [optional]
**limits_per_field** | Option<**bool**> | Determines whether the 'limit', 'limit_words', and 'limit_snippets' options operate as individual limits in each field of the document | [optional]
**no_match_size** | Option<**i32**> | If set to 1, allows an empty string to be returned as a highlighting result | [optional]
**order** | Option<**String**> | Sets the sorting order of highlighted snippets | [optional]
**pre_tags** | Option<**String**> | Text inserted before each highlighted snippet | [optional][default to <strong>]
**post_tags** | Option<**String**> | Text inserted after each highlighted snippet | [optional][default to </strong>]
**start_snippet_id** | Option<**i32**> | Sets the starting value of the %SNIPPET_ID% macro | [optional]
**use_boundaries** | Option<**bool**> | Defines whether to additionally break snippets by phrase boundary characters | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


