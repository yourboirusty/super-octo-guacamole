use avian2d::prelude::*;
pub const WALL_LAYER: u8 = 0b1000;
pub const PLAYER_LAYER: u8 = 0b0100;
pub const INTERACTIBLE_LAYER: u8 = 0b0010;
pub const CHECKPOINT_LAYER: u8 = 0b0001;

#[derive(PhysicsLayer, Default, Clone, Copy)]
pub enum LayerEnum {
    #[default]
    Wall,
    Player,
    Interactible,
    Checkpoint,
    None,
}
