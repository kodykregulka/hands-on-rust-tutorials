pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
	pub color: ColorPair,
	pub glyph: FontCharType,
}

//tag components
//empty struct containing no data indicating that an entity with this component is the player
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;