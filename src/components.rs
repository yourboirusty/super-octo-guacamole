use bevy::prelude::*;
#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

impl Player {
    pub fn new(handle: usize) -> Self {
        Self { handle }
    }
}
