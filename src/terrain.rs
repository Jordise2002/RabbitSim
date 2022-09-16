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
        let texture = texture_manager.get_texture(texture_name).unwrap();
        let mut draw_parameters = DrawTextureParams::default();
        draw_parameters.dest_size = Option::Some(Vec2{x:width, y:height});
        draw_texture_ex(*texture,x as f32, y as f32,WHITE,draw_parameters);
    }
}
