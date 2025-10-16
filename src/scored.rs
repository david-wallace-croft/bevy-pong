use super::scorer::Scorer;
use ::bevy::prelude::*;

#[derive(Message)]
pub struct Scored(pub Scorer);
