mod animal;
mod board;
mod vegetation;
mod drawable_trait;
mod terrain;
mod texture_manager;


use macroquad::prelude::*;
use crate::drawable_trait::Drawable;
use crate::miniquad::native::linux_x11::libx11::Screen;
use crate::texture_manager::TextureManager;

fn window_conf() -> Conf
{
    Conf{
        window_title: "Rabbit Sim".to_string(),
        window_width: 800,
        window_height: 800,
        high_dpi: false,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        icon: None,
        platform: Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    loop
    {
        let mut texture_manager = TextureManager::new().await;
        let board = board::Board::new(32,32);
        board.draw(0.,0., width,height,& mut texture_manager);
        next_frame().await;
    }
}
