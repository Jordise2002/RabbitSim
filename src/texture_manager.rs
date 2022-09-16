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

    pub fn get_texture(& mut self,texture_name: &str)-> Option<&Texture2D>
    {
        self.texture_map.get(texture_name)
    }
}