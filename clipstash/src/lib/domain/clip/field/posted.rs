use crate::domain::Time; 
use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(Self) -> Time {
        self.0
    }
}
