/**
This trait defines the shared behaviour of entities in the simulation
(animals and plants)
*/
pub trait Entity
{
    fn get_pos(&self) ->(usize,usize);
}