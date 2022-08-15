pub mod field;

use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClipError {
  #[error("Invalid password: {0}")]
  InvalidPassword(String),
  #[error("Invalid title: {0}")]
  InvalidTitle(String),
  #[error("Empty content")]
  EmptyContent,
  #[error("Invalid date: {0}")]
  InvalidDate(String),
  #[error("Date parse error: {0}")]
  DateParseError(#[from] chrono::ParseError),
  #[error("ID parse error: {0}")]
  IdParseError(#[from] uuid::Error),
  #[error("Hits parse error: {0}")]
  HitsParseError(#[from] std::num::TryFromIntError)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
  pub clip_id: field::ClipId,
  pub shortcode: field::Shortcode,
  pub content: field::Content,
  pub title: field::Title,
  pub posted: field::Posted,
  pub expires: field::Expires,
  pub password: field::Password,
  pub hits: field::Hits
}
