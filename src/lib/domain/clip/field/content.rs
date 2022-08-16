use crate::domain::clip::ClipError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
  pub fn new(content: &str) -> Result<Self, ClipError> {
    if !content.trim().is_empty() {
      Ok(Self(String::from(content)))
    } else {
      Err(ClipError::EmptyContent)
    }
  }

  pub fn into_inner(self) -> String {
    self.0
  }

  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }
}
