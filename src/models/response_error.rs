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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseError {
    one_of_0(Box<crate::models::ResponseErrorDetails>),
    /// Error message text returned in case of an error
    one_of_1(String),
}

impl Default for ResponseError {
    fn default() -> Self {
        Self::one_of_0(Default::default())
    }
}

