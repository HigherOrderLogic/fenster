use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Message {
  Foo,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum Response {
  Error { error: ResponseError },
  Success { content: ResponseContent },
}

impl Response {
  pub fn error(e: ResponseError) -> Self {
    Self::Error { error: e }
  }

  pub fn success(c: ResponseContent) -> Self {
    Self::Success { content: c }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "detail")]
pub enum ResponseError {
  MalformedMessage(String),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ResponseContent {
  Foo,
}
