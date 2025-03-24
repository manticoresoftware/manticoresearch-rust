use std::sync::Arc;
use manticoresearch::apis::configuration::Configuration;
use manticoresearch::apis::UtilsApi;
use manticoresearch::apis::UtilsApiClient;
use tokio;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[tokio::test]
async fn utils_api_basic_requests() {
    let api_config = Arc::new(Configuration::new());
    let utils_api = UtilsApiClient::new(api_config);
    let res = utils_api.sql("SHOW TABLES", Some(true)).await;
    println!("{:#?}", res);
    print_type_of(&res);
    //let result = add_two(2);
    //assert_eq!(res, );
}
