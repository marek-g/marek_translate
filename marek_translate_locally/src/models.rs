use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateRequest<'a> {
    pub id: i32,
    pub command: &'a str,
    pub data: TranslateRequestData<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateRequestData<'a> {
    pub src: &'a str,
    pub trg: &'a str,
    pub text: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateResponse {
    pub id: i32,
    pub success: bool,
    pub data: Option<TranslateResponseData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateResponseData {
    pub target: TranslateResponseTargetData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslateResponseTargetData {
    pub text: String,
}
