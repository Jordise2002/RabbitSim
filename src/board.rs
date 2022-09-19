use macroquad::prelude::*;
use ::rand::prelude::*;
use crate::animal::Animal;
use crate::drawable_trait::Drawable;
use crate::vegetation::Vegetation;
use crate::terrain::Terrain;
use crate::texture_manager::TextureManager;
use crate::tile::Tile;

pub const TILE_HEIGHT: f32 = 16.;

pub const TILE_WIDTH: f32 = 16.;



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
        for x in 0..self.size.0
        {
            for y in 0..self.size.1
            {
                self.tiles.push(Tile::new(x,y));
            }
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

    pub fn spawn_vegetation(&self,father: Box<dyn Vegetation>)
    {
        let mut rng = thread_rng();
        let mut pos = father.get_pos();
        loop
        {
            let random_direction = rng.gen_range(0..4);
            match random_direction
            {
                 0 =>
                    {
                        pos = (pos.0 + 1, pos.1);
                    }
                1 =>
                    {
                        pos = (pos.0 - 1, pos.1);
                    }
                2 =>
                    {
                        pos = (pos.0, pos.1 +1);
                    }
                3 =>
                    {
                        pos = (pos.0, pos.1 -1);
                    }
                _ => {
                    panic!("Wrong random number generated");
                }
            }

            match self.get_tile(pos.0, pos.1)
            {
                Some(tile) =>
                    {

                    }
                None =>
                    {
                        continue;
                    }
            }

        }
    }

}

impl Drawable for Board
{
    fn draw(&self, x: f32, y: f32, width: f32, height:f32, texture_manager: & mut TextureManager)
    {
        let tile_width = width / self.size.0 as f32;
        let title_height = height / self.size.1 as f32;
        for row in 0..self.size.0
        {
            for col in 0..self.size.1
            {
                self.get_tile(col,row).expect("No tile found").draw(x + col as f32* tile_width, y + row as f32*title_height,tile_width,title_height, texture_manager);
            }
        }
    }
}