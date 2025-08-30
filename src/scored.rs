use super::scorer::Scorer;
use ::bevy::prelude::*;

#[derive(Event)]
pub struct Scored(pub Scorer);
