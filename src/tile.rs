use std::borrow::BorrowMut;
use std::rc::Rc;
use crate::terrain::Terrain;
use crate::drawable_trait::Drawable;
use crate::texture_manager::TextureManager;
use crate::vegetation::{Grass, Vegetation};


pub struct Tile
{
    pos: (usize, usize),//col row
    terrain: Terrain,
    flora: Rc<dyn Vegetation>,
    //fauna: Box<dyn Animal>
}

impl Tile
{
    pub fn new(x:usize, y:usize) -> Tile
    {
        Tile{
            pos:(x, y),
            terrain: Terrain::Land,
            flora: Rc::new(Grass::new(x,y))
        }
    }
}

impl Drawable for Tile
{
    fn draw(&self,x: f32, y: f32,width: f32, height:f32, texture_manager: & mut TextureManager)
    {
        self.terrain.draw(x,y,width,height, texture_manager);
        //self.flora.draw(x,y,width,height, texture_manager);
        //self.fauna.draw(x,y,width,height, texture_manager);
    }
}