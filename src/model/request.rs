use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeySignRequest{
   pub pub_key: String,
    // principals: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeySignResponse {
    pub(crate) signed_key: String,
    // principals: Vec<String>
}