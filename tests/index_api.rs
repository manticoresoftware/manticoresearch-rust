use std::sync::Arc;
use tokio;
use manticoresearch::{
    apis::{configuration::Configuration, UtilsApi, UtilsApiClient, IndexApi, IndexApiClient},
    models::InsertDocumentRequest,
};
use std::collections::HashMap;

#[tokio::test]
async fn index_api_basic_requests() {
    let api_config = Arc::new(Configuration::new());
    let utils_api = UtilsApiClient::new(api_config.clone());
    let index_api = IndexApiClient::new(api_config.clone());

    // Drop the table if it exists
    let res = utils_api.sql("DROP TABLE IF EXISTS products", Some(true)).await;
    assert!(res.is_ok(), "DROP TABLE failed: {:?}", res.err());

    // Create table
    let res = utils_api
        .sql(
            "CREATE TABLE IF NOT EXISTS products (title text, price float, sizes multi, meta json, coeff float, tags1 multi, tags2 multi)",
            Some(true),
        )
        .await;
    assert!(res.is_ok(), "Failed to create table: {:?}", res.err());

    // Prepare document
    let mut doc = HashMap::new();
    doc.insert("title".to_string(), serde_json::json!("test"));
    doc.insert("tags1".to_string(), serde_json::json!([1, 2, 4, 5]));

    // Insert without specifying ID
    let insert_req = InsertDocumentRequest::new("products".to_string(), serde_json::json!(doc));
    let res = index_api.insert(insert_req).await;
    assert!(res.is_ok(), "Failed to insert document without ID: {:?}", res.err());
    
    // Insert with specific ID
    let mut insert_req = InsertDocumentRequest::new("products".to_string(), serde_json::json!(doc));
    insert_req.id = Some(100);
    let res = index_api.insert(insert_req).await;
    assert!(res.is_ok(), "Failed to insert document with ID: {:?}", res.err());

    let insert_response = res.unwrap();
    println!("Insert response with ID 100: {:?}", insert_response);

    // Drop the table if it exists
    let res = utils_api.sql("DROP TABLE IF EXISTS products", Some(true)).await;
    assert!(res.is_ok(), "DROP TABLE failed: {:?}", res.err());
    
    
    // Create table
    let res = utils_api
        .sql(
            "CREATE TABLE IF NOT EXISTS products (title text, price float, sizes multi, meta json, coeff float, tags1 multi, tags2 multi)",
            Some(true),
        )
        .await;
    assert!(res.is_ok(), "Failed to create table: {:?}", res.err());

    // Bulk insert
    let insert_body = r#"{"insert": {"table": "products", "id": 3, "doc": {"title": "Crossbody \"Bag with Tassel", "price": 19.85}}}
{"insert": {"table": "products", "id": 4, "doc": {"title": "microfiber sheet set", "price": 19.99}}}
{"insert": {"table": "products", "id": 5, "doc": {"title": "CPet Hair Remover Glove", "price": 7.99}}}
"#;
    let res = index_api.bulk(insert_body).await;
    assert!(res.is_ok(), "Bulk insert failed: {:?}", res.err());

    let bulk_insert_response = res.unwrap();
    println!("Bulk insert response with ID 100: {:?}", bulk_insert_response);

    // Bulk update
    let update_body = r#"{"update": { "table" : "products", "doc": { "coeff" : 1000 }, "query": { "range": { "price": { "gte": 1000 } } } } }
{ "update" : { "table" : "products", "doc": { "coeff" : 0 }, "query": { "range": { "price": { "lt": 1000 } } } } }
"#;
    let res = index_api.bulk(update_body).await;
    assert!(res.is_ok(), "Bulk update failed: {:?}", res.err());
}