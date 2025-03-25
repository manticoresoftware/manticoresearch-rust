use std::sync::Arc;
use manticoresearch::{apis::configuration::Configuration, apis::UtilsApi, apis::UtilsApiClient, models::SqlResponse};

use tokio;


#[tokio::test]
async fn utils_api_basic_requests() {
    let api_config = Arc::new(Configuration::new());
    let utils_api = UtilsApiClient::new(api_config);

    // DROP TABLE IF EXISTS
    let res = utils_api.sql("DROP TABLE IF EXISTS products", Some(true)).await;
    assert!(res.is_ok(), "DROP TABLE failed: {:?}", res.err());

    // CREATE TABLE
    let create_query = "CREATE TABLE IF NOT EXISTS products (title text, price float, sizes multi, meta json, coeff float, tags1 multi, tags2 multi)";
    let res = utils_api.sql(create_query, Some(true)).await;
    assert!(res.is_ok(), "CREATE TABLE failed: {:?}", res.err());

    // SELECT with parse=true
    let res = utils_api.sql("SELECT * FROM products", Some(true)).await;
    assert!(res.is_ok(), "SELECT (parse=true) failed: {:?}", res.err());

    // SELECT with parse=false
    let res = utils_api.sql("SELECT * FROM products", Some(false)).await;
    assert!(res.is_ok(), "SELECT (parse=false) failed: {:?}", res.err());

    // SHOW TABLES
    let res = utils_api.sql("SHOW TABLES", Some(true)).await;
    assert!(res.is_ok(), "SHOW TABLES failed: {:?}", res.err());

    let result = res.unwrap();
    let is_arr_response = matches!(result, SqlResponse::one_of_0 { .. });
    assert!(is_arr_response == true);

    if let SqlResponse::one_of_0(objs) = result {
        println!("value: {:#?}", objs[0]);
    }
}
