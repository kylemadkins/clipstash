use derive_more::Constructor;
use serde::{Serialize, Deserialize};

#[derive(Clone, Constructor, Debug, Serialize, Deserialize)]
pub struct Hits(u64);

impl Hits {
  /*

  The `Constructor` trait from `derive_more` generates this code for us:

  pub fn new(hits: u64) -> Self {
    Self(hits)
  }

  */

  pub fn into_inner(self) -> u64 {
    self.0
  }
}
