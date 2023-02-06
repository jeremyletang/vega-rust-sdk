use crate::commands::Command;
use rand;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub sending_mode: String,
    pub transaction: Command,
    public_key: String,
}

impl Params {
    pub fn new(cmd: Command, pubkey: &str) -> Params {
        return Params {
            sending_mode: "TYPE_SYNC".to_string(),
            transaction: cmd,
            public_key: pubkey.to_string(),
        };
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(rename = "jsonrpc")]
    pub version: String,
    pub method: String,
    pub params: Option<Params>,
    pub id: String,
}

impl Request {
    pub fn new_send_transaction(cmd: Command, pubkey: &str) -> Request {
        return Request {
            version: "2.0".to_string(),
            method: "client.send_transaction".to_string(),
            params: Some(Params::new(cmd, pubkey)),
            id: rand::random::<u64>().to_string(),
        };
    }

    pub fn new_sign_transaction(cmd: Command, pubkey: &str) -> Request {
        return Request {
            version: "2.0".to_string(),
            method: "client.sign_transaction".to_string(),
            params: Some(Params::new(cmd, pubkey)),
            id: rand::random::<u64>().to_string(),
        };
    }

    pub fn new_list_keys() -> Request {
        return Request {
            version: "2.0".to_string(),
            method: "client.list_keys".to_string(),
            params: None,
            id: rand::random::<u64>().to_string(),
        };
    }
}
