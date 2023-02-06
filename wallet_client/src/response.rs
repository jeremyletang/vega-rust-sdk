use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    #[serde(rename = "jsonrpc")]
    pub version: String,
    pub result: Option<T>,
    pub error: Option<WalletError>,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeysResponse {
    pub keys: Vec<Key>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    pub name: String,
    pub public_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletError {
    pub code: i64,
    pub message: String,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignResponse {
    pub transaction: Transaction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub input_data: String,
    pub signature: Signature,
    #[serde(rename = "From")]
    pub from: From,
    pub version: u64,
    pub pow: Pow,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub value: String,
    pub algo: String,
    pub version: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct From {
    #[serde(rename = "PubKey")]
    pub pub_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pow {
    pub tid: String,
    pub nonce: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendResponse {
    pub received_at: DateTime<Utc>,
    pub sent_at: DateTime<Utc>,
    pub transaction_hash: String,
    pub transaction: Transaction,
}
