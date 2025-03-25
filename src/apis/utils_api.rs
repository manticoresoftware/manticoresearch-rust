/*
 * Manticore Search Client
 *
 * Сlient for Manticore Search. 
 *
 * The version of the OpenAPI document: 5.0.0
 * Contact: info@manticoresearch.com
 * Generated by: https://openapi-generator.tech
 */

use std::sync::Arc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use futures::Future;

use crate::models;
use super::{Error, configuration};
use super::request as __internal_request;

pub struct UtilsApiClient<C: Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> UtilsApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> UtilsApiClient<C> {
        UtilsApiClient {
            configuration,
        }
    }
}

pub trait UtilsApi: Send + Sync {
    fn sql(&self, body: &str, raw_response: Option<bool>) -> Pin<Box<dyn Future<Output = Result<crate::models::SqlResponse, Error>> + Send>>;
}

impl<C: Connect>UtilsApi for UtilsApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn sql(&self, body: &str, raw_response: Option<bool>) -> Pin<Box<dyn Future<Output = Result<crate::models::SqlResponse, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/sql".to_string())
        ;
        if let Some(ref s) = raw_response {
            let query_value = s.to_string();
            req = req.with_query_param("raw_response".to_string(), query_value);
        }
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

}
