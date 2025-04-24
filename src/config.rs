use bevy_matchbox::prelude::PeerId;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;
pub const INPUT_USE: u8 = 1 << 5;

pub type MultiplayerConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;
