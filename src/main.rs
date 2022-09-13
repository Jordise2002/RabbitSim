mod animal;
mod board;
mod vegetation;
mod drawable_trait;
mod terrain;
mod texture_manager;


use macroquad::prelude::*;
use crate::drawable_trait::Drawable;
use crate::texture_manager::TextureManager;

#[macroquad::main("Rabbit Sim")]
async fn main() {

    loop
    {
        let mut texture_manager = TextureManager::new().await;
        let board = board::Board::new(4,4);
        board.draw(0,0,& mut texture_manager);
        next_frame().await;
    }
}
