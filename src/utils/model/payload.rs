//! Request payload models
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PayloadCharacterAdd {
    pub player: String,
    pub character: i64,
}
