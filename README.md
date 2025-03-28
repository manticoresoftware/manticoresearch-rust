# Manticore Rust client

❗ WARNING: this is a development version of the client. The latest release's readme is https://github.com/manticoresoftware/manticoresearch-rust/tree/1.0.0


Сlient for Manticore Search.


For more information, please visit [https://manticoresearch.com/contact-us/](https://manticoresearch.com/contact-us/)

## Compatibility table

| **manticoresearch-rust* | **Manticore Search**                | **Compatibility**       |
| ------------------------| ----------------------------------- | ------------------------|
| `manticoresearch-dev`   | `dev` (latest development version)  | ✅ Fully Compatible     |
| 1.0.0 or newer          | 9.2.14 or newer                     | ✅ Fully Compatible     |

## Installation

Put the package under your project folder in a directory named `manticoresearch` and add the following to `Cargo.toml` under `[dependencies]`:

```
manticoresearch = { path = "./manticoresearch" }
```

## Documentation for API Endpoints

All URIs are relative to *http://127.0.0.1:9308*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*IndexApi* | [**bulk**](docs/IndexApi.md#bulk) | **Post** /bulk | Bulk table operations
*IndexApi* | [**delete**](docs/IndexApi.md#delete) | **Post** /delete | Delete a document in a table
*IndexApi* | [**insert**](docs/IndexApi.md#insert) | **Post** /insert | Create a new document in a table
*IndexApi* | [**partial_replace**](docs/IndexApi.md#partial_replace) | **Post** /{table}/_update/{id} | Partially replaces a document in a table
*IndexApi* | [**replace**](docs/IndexApi.md#replace) | **Post** /replace | Replace new document in a table
*IndexApi* | [**update**](docs/IndexApi.md#update) | **Post** /update | Update a document in a table
*SearchApi* | [**autocomplete**](docs/SearchApi.md#autocomplete) | **Post** /autocomplete | Performs an autocomplete search on a table
*SearchApi* | [**percolate**](docs/SearchApi.md#percolate) | **Post** /pq/{table}/search | Perform reverse search on a percolate table
*SearchApi* | [**search**](docs/SearchApi.md#search) | **Post** /search | Performs a search on a table
*UtilsApi* | [**sql**](docs/UtilsApi.md#sql) | **Post** /sql | Perform SQL requests


## Documentation For Models

 - [AggComposite](docs/AggComposite.md)
 - [AggCompositeSource](docs/AggCompositeSource.md)
 - [AggCompositeTerm](docs/AggCompositeTerm.md)
 - [AggTerms](docs/AggTerms.md)
 - [Aggregation](docs/Aggregation.md)
 - [AutocompleteRequest](docs/AutocompleteRequest.md)
 - [BoolFilter](docs/BoolFilter.md)
 - [BulkResponse](docs/BulkResponse.md)
 - [DeleteDocumentRequest](docs/DeleteDocumentRequest.md)
 - [DeleteResponse](docs/DeleteResponse.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [FulltextFilter](docs/FulltextFilter.md)
 - [GeoDistance](docs/GeoDistance.md)
 - [GeoDistanceLocationAnchor](docs/GeoDistanceLocationAnchor.md)
 - [Highlight](docs/Highlight.md)
 - [HighlightFieldOption](docs/HighlightFieldOption.md)
 - [HitsHits](docs/HitsHits.md)
 - [InsertDocumentRequest](docs/InsertDocumentRequest.md)
 - [Join](docs/Join.md)
 - [JoinCond](docs/JoinCond.md)
 - [JoinOn](docs/JoinOn.md)
 - [KnnQuery](docs/KnnQuery.md)
 - [Match](docs/Match.md)
 - [MatchAll](docs/MatchAll.md)
 - [PercolateRequest](docs/PercolateRequest.md)
 - [PercolateRequestQuery](docs/PercolateRequestQuery.md)
 - [QueryFilter](docs/QueryFilter.md)
 - [Range](docs/Range.md)
 - [ReplaceDocumentRequest](docs/ReplaceDocumentRequest.md)
 - [ResponseError](docs/ResponseError.md)
 - [ResponseErrorDetails](docs/ResponseErrorDetails.md)
 - [SearchQuery](docs/SearchQuery.md)
 - [SearchRequest](docs/SearchRequest.md)
 - [SearchResponse](docs/SearchResponse.md)
 - [SearchResponseHits](docs/SearchResponseHits.md)
 - [SourceRules](docs/SourceRules.md)
 - [SqlObjResponse](docs/SqlObjResponse.md)
 - [SqlResponse](docs/SqlResponse.md)
 - [SuccessResponse](docs/SuccessResponse.md)
 - [UpdateDocumentRequest](docs/UpdateDocumentRequest.md)
 - [UpdateResponse](docs/UpdateResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Getting started

```rust

use std::sync::Arc;
use std::collections::HashMap;
use serde_json;
use http_body_util::BodyExt;
use tokio;
use manticoresearch::{
    apis::{
        {Error,configuration::Configuration,IndexApi,IndexApiClient,SearchApi,SearchApiClient,UtilsApi,UtilsApiClient}
    },
    models::{SearchRequest,SearchQuery,Highlight}
};


#[tokio::main]
async fn main() {
    let api_config = Arc::new(Configuration::new());
    let utils_api = UtilsApiClient::new(api_config.clone());
    let index_api = IndexApiClient::new(api_config.clone());
    let search_api = SearchApiClient::new(api_config.clone());

    // Drop table if it exists
    let _ = utils_api.sql("DROP TABLE IF EXISTS movies", Some(true)).await;
    
    // Create table
    let _ = utils_api
        .sql(
            "CREATE TABLE IF NOT EXISTS movies (title text, plot text, _year integer, rating float, cat string, code multi, type_vector float_vector knn_type='hnsw' knn_dims='3' hnsw_similarity='l2')",
            Some(true),
        )
        .await;
    
    // Bulk insert documents
    let bulk_body = r#"{"insert": {"table" : "movies", "id" : 1, "doc" : {"title" : "Star Trek 2: Nemesis", "plot": "The Enterprise is diverted to the Romulan homeworld Romulus, supposedly because they want to negotiate a peace treaty. Captain Picard and his crew discover a serious threat to the Federation once Praetor Shinzon plans to attack Earth.", "_year": 2002, "rating": 6.4, "cat": "R", "code": [1,2,3], "type_vector": [0.2, 1.4, -2.3]}}}
{"insert": {"table" : "movies", "id" : 2, "doc" : {"title" : "Star Trek 1: Nemesis", "plot": "The Enterprise is diverted to the Romulan homeworld Romulus, supposedly because they want to negotiate a peace treaty. Captain Picard and his crew discover a serious threat to the Federation once Praetor Shinzon plans to attack Earth.", "_year": 2001, "rating": 6.5, "cat": "PG-13", "code": [1,12,3], "type_vector": [0.8, 0.4, 1.3]}}}
{"insert": {"table" : "movies", "id" : 3, "doc" : {"title" : "Star Trek 3: Nemesis", "plot": "The Enterprise is diverted to the Romulan homeworld Romulus, supposedly because they want to negotiate a peace treaty. Captain Picard and his crew discover a serious threat to the Federation once Praetor Shinzon plans to attack Earth.", "_year": 2003, "rating": 6.6, "cat": "R", "code": [11,2,3], "type_vector": [1.5, -1.0, 1.6]}}}
{"insert": {"table" : "movies", "id" : 4, "doc" : {"title" : "Star Trek 4: Nemesis", "plot": "The Enterprise is diverted to the Romulan homeworld Romulus, supposedly because they want to negotiate a peace treaty. Captain Picard and his crew discover a serious threat to the Federation once Praetor Shinzon plans to attack Earth.", "_year": 2003, "rating": 6.0, "cat": "R", "code": [1,2,4], "type_vector": [0.4, 2.4, 0.9]}}}
"#;

    let _ = index_api.bulk(bulk_body).await;
    
    // Prepare search request
    let query = SearchQuery {
        query_string: Some(serde_json::json!("Star").into()),
        ..Default::default()
    };

    let highlight = Highlight {
        fields: Some(serde_json::json!(["title"]).into()),
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
    
    let _ = match res {
        Ok(result) => {
            println!("Search result: {:?}", result)        
        },
        Err(error) => {
            if let Error::Api(error_info) = error {
                let body_bytes = error_info.body.collect().await.expect("ERROR RESPONSE").to_bytes();
                println!("Error response: {:?}", String::from_utf8(body_bytes.to_vec()).unwrap())
            }
        }
    };
    
}


```
