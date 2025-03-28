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

/// AggCompositeTerm : Object representing a term to be used in composite aggregation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggCompositeTerm {
    /// Name of field to operate with
    #[serde(rename = "field")]
    pub field: String,
}

impl AggCompositeTerm {
    /// Object representing a term to be used in composite aggregation.
    pub fn new(field: String) -> AggCompositeTerm {
        AggCompositeTerm {
            field,
        }
    }
}

