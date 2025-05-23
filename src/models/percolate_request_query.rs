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
pub struct PercolateRequestQuery {
    /// Object representing the document to percolate
    #[serde(rename = "percolate")]
    pub percolate: serde_json::Value,
}

impl PercolateRequestQuery {
    pub fn new(percolate: serde_json::Value) -> PercolateRequestQuery {
        PercolateRequestQuery {
            percolate,
        }
    }
}

