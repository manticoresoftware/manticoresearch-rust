use std::sync::Arc;
use manticoresearch::{
    apis::{
        {configuration::Configuration,IndexApi,IndexApiClient,SearchApi,SearchApiClient,UtilsApi,UtilsApiClient}
    },
    models::{SearchRequest,SearchQuery,Highlight,HighlightFields}
};
use std::collections::HashMap;
use tokio;

#[tokio::test]
async fn search_api_basic_requests() {
    let api_config = Arc::new(Configuration::new());
    let utils_api = UtilsApiClient::new(api_config.clone());
    let index_api = IndexApiClient::new(api_config.clone());
    let search_api = SearchApiClient::new(api_config.clone());

    // Drop table if it exists
    let res = utils_api.sql("DROP TABLE IF EXISTS movies", Some(true)).await;
    assert!(res.is_ok(), "Failed to drop table: {:?}", res.err());

    // Create table
    let res = utils_api
        .sql(
            "CREATE TABLE IF NOT EXISTS movies (title text, plot text, _year integer, rating float, cat string, code multi, type_vector float_vector knn_type='hnsw' knn_dims='3' hnsw_similarity='l2')",
            Some(true),
        )
        .await;
    assert!(res.is_ok(), "Failed to create table: {:?}", res.err());

    // Bulk insert documents
    let bulk_body = r#"{"insert": {"table" : "movies", "id" : 1, "doc" : {"title" : "Star Trek 2: Nemesis", "plot": "The Enterprise is diverted to the Romulan homeworld Romulus, supposedly because they want to negotiate a peace treaty. Captain Picard and his crew discover a serious threat to the Federation once Praetor Shinzon plans to attack Earth.", "_year": 2002, "rating": 6.4, "cat": "R", "code": [1,2,3], "type_vector": [0.2, 1.4, -2.3]}}}
{"insert": {"table" : "movies", "id" : 2, "doc" : {"title" : "Star Trek 1: Nemesis", "plot": "The Enterprise is diverted to the Romulan homeworld Romulus, supposedly because they want to negotiate a peace treaty. Captain Picard and his crew discover a serious threat to the Federation once Praetor Shinzon plans to attack Earth.", "_year": 2001, "rating": 6.5, "cat": "PG-13", "code": [1,12,3], "type_vector": [0.8, 0.4, 1.3]}}}
{"insert": {"table" : "movies", "id" : 3, "doc" : {"title" : "Star Trek 3: Nemesis", "plot": "The Enterprise is diverted to the Romulan homeworld Romulus, supposedly because they want to negotiate a peace treaty. Captain Picard and his crew discover a serious threat to the Federation once Praetor Shinzon plans to attack Earth.", "_year": 2003, "rating": 6.6, "cat": "R", "code": [11,2,3], "type_vector": [1.5, -1.0, 1.6]}}}
{"insert": {"table" : "movies", "id" : 4, "doc" : {"title" : "Star Trek 4: Nemesis", "plot": "The Enterprise is diverted to the Romulan homeworld Romulus, supposedly because they want to negotiate a peace treaty. Captain Picard and his crew discover a serious threat to the Federation once Praetor Shinzon plans to attack Earth.", "_year": 2003, "rating": 6.0, "cat": "R", "code": [1,2,4], "type_vector": [0.4, 2.4, 0.9]}}}
"#;

    let res = index_api.bulk(bulk_body).await;
    assert!(res.is_ok(), "Bulk insert failed: {:?}", res.err());

    // Prepare search request
    let query = SearchQuery {
        query_string: Some("Star".to_string()),
        ..Default::default()
    };

    let highlight_fields = HighlightFields::ArrayVecString(vec!["title".to_string()]);

    let highlight = Highlight {
        fields: Some(Box::new(highlight_fields)),
        ..Default::default()
    };

    let mut options = HashMap::new();
    options.insert("cutoff".to_string(), serde_json::json!(5));
    options.insert("ranker".to_string(), serde_json::json!("bm25"));

    let search_request = SearchRequest {
        table: "movies".to_string(),
        query: Some(Box::new(query)),
        highlight: Some(Box::new(highlight)),
        options: Some(serde_json::json!(options)),
        ..Default::default()
    };

    // Perform search
    let res = search_api.search(search_request).await;
    assert!(res.is_ok(), "Search failed: {:?}", res.err());

    let result = res.unwrap();
    println!("Search result: {:?}", result);

}

