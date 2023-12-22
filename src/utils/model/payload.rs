//! Request payload models
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PayloadCharacterAdd {
    pub player: i64,
    pub character: i64,
}
