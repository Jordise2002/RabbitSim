use std::collections::HashMap;
use macroquad::prelude::*;

const TEXTURE_NAMES: [&'static str;2] = ["Water", "Land"];
pub struct TextureManager
{
    texture_map: HashMap<String,Texture2D>
}

impl TextureManager
{
    pub async fn new() -> TextureManager
    {
        let mut t = TextureManager
        {
            texture_map:HashMap::new()
        };
        t.load_textures().await;
        t
    }

    async fn load_textures(& mut self)
    {
        for name in TEXTURE_NAMES
        {
            let texture = load_texture(format!("resources/{}.png",name).as_str()).await.expect("Error while reading file");
            self.texture_map.insert(name.to_string(),texture);
        }
    }

    fn get_texture(& mut self,texture_name: &str)-> Option<&Texture2D>
    {
        self.texture_map.get(texture_name)
    }

    pub fn draw_texture(& mut self,x: f32, y: f32, width: f32, height: f32, texture_name: &str)
    {
        let texture = *self.get_texture(texture_name).expect(format!("{} texture doesn't exist",texture_name).as_str());

        let mut draw_parameters = DrawTextureParams::default();
        draw_parameters.dest_size = Option::Some(Vec2{x:width, y:height});

        draw_texture_ex(texture,x as f32, y as f32,WHITE,draw_parameters);
    }
}