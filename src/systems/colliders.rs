use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::controller::collision_masks::LayerEnum;
#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: LinearVelocity,
    pub constraints: LockedAxes,
    pub max_velocity: MaxLinearSpeed,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderDensity,
}

#[derive(Clone, Copy)]
pub enum CharacterCollider {
    Player,
}

// collider: Collider::rectangle(16., 20.),
// rigid_body: RigidBody::Dynamic,
// constraints,
// max_velocity: MaxLinearSpeed(100.0),
// gravity_scale: GravityScale(1.),
// friction: Friction::new(0.),
// density: ColliderDensity(100.0),
// ..Default::default()

impl From<CharacterCollider> for Collider {
    fn from(value: CharacterCollider) -> Self {
        match value {
            CharacterCollider::Player => Collider::capsule(4.0, 5.),
        }
    }
}

impl From<CharacterCollider> for CollisionLayers {
    fn from(value: CharacterCollider) -> Self {
        match value {
            CharacterCollider::Player => CollisionLayers::new(
                LayerEnum::Player,
                [
                    LayerEnum::Wall,
                    LayerEnum::Interactible,
                    LayerEnum::Checkpoint,
                ],
            ),
        }
    }
}
