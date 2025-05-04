use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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

pub enum CharacterCollider {
    Player,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(value: &EntityInstance) -> Self {
        let constraints = LockedAxes::ROTATION_LOCKED;
        let tile = value.tile.expect("no tile attached");
        match value.identifier.as_ref() {
            "Wall" => ColliderBundle {
                collider: Collider::rectangle(tile.w as f32, tile.h as f32),
                rigid_body: RigidBody::Static,
                friction: Friction {
                    dynamic_coefficient: 1.,
                    static_coefficient: 1.,
                    combine_rule: CoefficientCombine::Min,
                },
                constraints,
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<CharacterCollider> for Collider {
    fn from(value: CharacterCollider) -> Self {
        match value {
            CharacterCollider::Player => Collider::rectangle(16., 20.),
        }
    }
}

impl From<CharacterCollider> for ColliderBundle {
    fn from(value: CharacterCollider) -> Self {
        let constraints = LockedAxes::ROTATION_LOCKED;
        match value {
            CharacterCollider::Player => ColliderBundle {
                collider: Collider::rectangle(16., 20.),
                rigid_body: RigidBody::Dynamic,
                constraints,
                max_velocity: MaxLinearSpeed(100.0),
                gravity_scale: GravityScale(1.),
                friction: Friction::new(0.),
                density: ColliderDensity(100.0),
                ..Default::default()
            },
        }
    }
}
