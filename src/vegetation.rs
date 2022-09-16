use crate::board::Board;
use crate::{Drawable, TextureManager};

pub trait Vegetation
{
    fn grow(&self, board: & mut Board);
}

struct Grass
{
    grow_phase:u32
}

impl