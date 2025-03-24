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
pub struct BoolFilter {
    /// Query clauses that must match for the document to be included
    #[serde(rename = "must", skip_serializing_if = "Option::is_none")]
    pub must: Option<Vec<crate::models::QueryFilter>>,
    /// Query clauses that must not match for the document to be included
    #[serde(rename = "must_not", skip_serializing_if = "Option::is_none")]
    pub must_not: Option<Vec<crate::models::QueryFilter>>,
    /// Query clauses that should be matched, but are not required
    #[serde(rename = "should", skip_serializing_if = "Option::is_none")]
    pub should: Option<Vec<crate::models::QueryFilter>>,
}

impl BoolFilter {
    pub fn new() -> BoolFilter {
        BoolFilter {
            must: None,
            must_not: None,
            should: None,
        }
    }
}

