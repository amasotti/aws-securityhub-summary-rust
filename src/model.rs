use serde::{Deserialize, Serialize};

/// Request struct
#[derive(Deserialize, Debug)]
pub struct Request {
    pub id: String,
    pub region: String,
    // TODO: Add more fields
}

/// Response struct
#[derive(Serialize)]
pub struct Response {
    pub req_id: String,
    pub msg: String,
}
