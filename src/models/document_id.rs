use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WireDocumentId {
    Num(u64),
    Uuid(String),
}

impl WireDocumentId {
    pub fn split(self) -> (Option<u64>, Option<String>) {
        match self {
            WireDocumentId::Num(n) => (Some(n), None),
            WireDocumentId::Uuid(s) => (None, Some(s)),
        }
    }
    pub fn merge(id: Option<u64>, uuid: Option<String>) -> Option<Self> {
        if let Some(u) = uuid {
            return Some(WireDocumentId::Uuid(u));
        }
        id.map(WireDocumentId::Num)
    }
}
