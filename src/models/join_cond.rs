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

/// JoinCond : Object representing the conditions used to perform the join operation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JoinCond {
    /// Field to join on
    #[serde(rename = "field")]
    pub field: String,
    /// Joined table
    #[serde(rename = "table")]
    pub table: String,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<models::FulltextFilter>>,
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<serde_json::Value>>,
}

impl JoinCond {
    /// Object representing the conditions used to perform the join operation
    pub fn new(field: String, table: String) -> JoinCond {
        JoinCond {
            field,
            table,
            query: None,
            r#type: None,
        }
    }
}

