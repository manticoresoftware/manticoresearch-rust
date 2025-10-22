use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    index_api: Box<dyn crate::apis::IndexApi>,
    search_api: Box<dyn crate::apis::SearchApi>,
    utils_api: Box<dyn crate::apis::UtilsApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            index_api: Box::new(crate::apis::IndexApiClient::new(rc.clone())),
            search_api: Box::new(crate::apis::SearchApiClient::new(rc.clone())),
            utils_api: Box::new(crate::apis::UtilsApiClient::new(rc.clone())),
        }
    }

    pub fn index_api(&self) -> &dyn crate::apis::IndexApi{
        self.index_api.as_ref()
    }

    pub fn search_api(&self) -> &dyn crate::apis::SearchApi{
        self.search_api.as_ref()
    }

    pub fn utils_api(&self) -> &dyn crate::apis::UtilsApi{
        self.utils_api.as_ref()
    }

}
