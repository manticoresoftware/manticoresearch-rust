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

/// DeleteResponse : Response object for successful delete request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteResponse {
    /// The name of the table from which the document was deleted
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    /// Number of documents deleted
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i32>,
    /// The ID of the deleted document. If multiple documents are deleted, the ID of the first deleted document is returned
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Indicates whether any documents to be deleted were found
    #[serde(rename = "found", skip_serializing_if = "Option::is_none")]
    pub found: Option<bool>,
    /// Result of the delete operation, typically 'deleted'
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl DeleteResponse {
    /// Response object for successful delete request
    pub fn new() -> DeleteResponse {
        DeleteResponse {
            table: None,
            deleted: None,
            id: None,
            found: None,
            result: None,
        }
    }
}

