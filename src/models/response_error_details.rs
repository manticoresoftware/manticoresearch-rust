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

/// ResponseErrorDetails : Detailed error information returned in case of an error response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseErrorDetails {
    /// Type or category of the error
    #[serde(rename = "type")]
    pub r#type: String,
    /// Detailed explanation of why the error occurred
    #[serde(rename = "reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// The table related to the error, if applicable
    #[serde(rename = "table", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub table: Option<Option<String>>,
}

impl ResponseErrorDetails {
    /// Detailed error information returned in case of an error response
    pub fn new(r#type: String) -> ResponseErrorDetails {
        ResponseErrorDetails {
            r#type,
            reason: None,
            table: None,
        }
    }
}

