pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Missile;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}
