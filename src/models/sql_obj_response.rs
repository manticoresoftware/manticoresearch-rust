/*
 * Manticore Search Client
 *
 * Сlient for Manticore Search. 
 *
 * The version of the OpenAPI document: 5.0.0
 * Contact: info@manticoresearch.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlObjResponse {
    #[serde(rename = "hits")]
    pub hits: serde_json::Value,
}

impl SqlObjResponse {
    pub fn new(hits: serde_json::Value) -> SqlObjResponse {
        SqlObjResponse {
            hits,
        }
    }
}

