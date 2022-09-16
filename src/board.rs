use macroquad::prelude::*;
use crate::animal::Animal;
use crate::drawable_trait::Drawable;
use crate::vegetation::Vegetation;
use crate::terrain::Terrain;
use crate::texture_manager::TextureManager;

pub const TILE_HEIGHT: f32 = 16.;

pub const TILE_WIDTH: f32 = 16.;


struct Tile
{
    terrain: Terrain,
    flora: Box<dyn Vegetation>,
    //fauna: Box<dyn Animal>
}

impl Tile
{
    pub fn new() -> Tile
    {
        Tile{
            terrain: Terrain::Land,
            flora
        }
    }
}

impl Drawable for Tile
{
    fn draw(&self,x: f32, y: f32,width: f32, height:f32, texture_manager: & mut TextureManager)
    {
        self.terrain.draw(x,y,width,height, texture_manager);
        self.flora.draw(x,y,width,height, texture_manager);
        self.fauna.draw(x,y,width,height, texture_manager);
    }
}
pub struct Board{
    size: (usize, usize),//(X(col), Y(row))
    tiles: Vec<Tile>,
}

impl Board
{
    pub fn new(n_col:usize, n_row:usize) -> Board
    {
        let mut b = Board
        {
            size:(n_col, n_row),
            tiles: Vec::new()
        };
        b.populate();
        b
    }

    fn populate(& mut self)
    {
        for i in 0..(self.size.0 * self.size.1)
        {
            self.tiles.push(Tile::new());
        }
    }


    fn get_pos(&self,col: usize, row: usize) -> Option<usize>
    {
        if col < self.size.0 && row < self.size.1
        {
            return Option::Some(row * self.size.0 + col);
        }
        Option::None
    }

    pub fn get_tile(&self, col:usize, row:usize) -> Option<&Tile>
    {
        self.tiles.get(self.get_pos(col, row)?)
    }

}

impl Drawable for Board
{
    fn draw(&self, x: f32, y: f32, width: f32, height:f32, texture_manager: & mut TextureManager)
    {
        let tile_width = width / self.size.0 as f32;
        let title_height = width / self.size.1 as f32;
        for row in 0..self.size.0
        {
            for col in 0..self.size.1
            {
                self.get_tile(col,row).expect("No tile found").draw(x + col as f32* tile_width, y + row as f32*title_height,tile_width,title_height, texture_manager);
            }
        }
    }
}