use crate::board::Board;
use crate::{Drawable, TextureManager};
use rand::prelude::*;
use crate::entity_trait::Entity;

pub trait Vegetation: Drawable + Entity
{
    fn grow(self, board: & Board);
    fn add_grow_phase(self);
}

pub struct Grass
{
    pos:(usize,usize),
    grow_phase:u32
}

impl Drawable for Grass {
    fn draw(&self, x: f32, y: f32, width: f32, height: f32, texture_manager: &mut TextureManager)
    {
        let mut texture_name = "";
        match self.grow_phase {
            0 => {
                texture_name = "Grass0";
            }
            1 => {
                texture_name = "Grass1";
            }
            2 => {
                texture_name = "Grass2";
            }
            3 => {
                texture_name = "Grass3";
            }
            _ => {
                texture_name = "Grass4";
            }
        }
        texture_manager.draw_texture(x,y,width,height,texture_name);
    }
}

impl Entity for Grass
{
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}

impl Vegetation for Grass
{
    fn grow(self, board: &Board) {
        if self.grow_phase < 4
        {
            self.add_grow_phase();
        }
        else
        {
            board.spawn_vegetation(Box::new(self));
        }
    }
    fn add_grow_phase(mut self)
    {
         self.grow_phase = self.grow_phase + 1;
    }
}

impl Grass{
    pub fn new(x:usize,y:usize) -> Grass {
        Grass{
            pos:(x,y),
            grow_phase: 0
        }
    }

    pub fn new_with_grow(x:usize, y:usize, grow_phase: u32) -> Grass {
        Grass{
            pos:(x,y),
            grow_phase
        }
    }
}