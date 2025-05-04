use bevy_matchbox::prelude::PeerId;

pub const TARGET_FPS: usize = 60;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;
pub const INPUT_USE: u8 = 1 << 5;

pub const PLAYER_Z: f32 = 9.0;

pub type MultiplayerConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

pub const LEVEL_IIDS: [&str; 1] = ["2d3efb50-1030-11f0-bddd-f1f4e985be26"];
