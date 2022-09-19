use macroquad::color::WHITE;
use macroquad::prelude::{draw_texture_ex, DrawTextureParams};
use crate::board::TILE_WIDTH;
use crate::drawable_trait::Drawable;
use crate::texture_manager::TextureManager;
use crate::Vec2;

pub enum Terrain
{
    Water,
    Land
}

impl Drawable for Terrain{
    fn draw(&self, x:f32, y: f32,width:f32, height:f32, texture_manager: & mut TextureManager)
    {
        let mut texture_name = "";
        match self{
            Terrain::Water =>
                {
                    texture_name = "Water";
                }
            Terrain::Land =>
                {
                    texture_name = "Land";
                }
        }
        texture_manager.draw_texture(x,y,width, height,texture_name);
    }
}
