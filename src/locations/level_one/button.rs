use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::locations::{
        level_one::{BUTTON_HITBOX_X_OFFSET, BUTTON_POSITION},
        LEVEL_SCALE,
    },
    locations::Location,
};

/// # Note
///
/// Instead of creating struct to differenciate Doors,
/// Link Doors and Button:
/// `Button { pub linked_doors: Vec<Entity> }`
#[derive(Component)]
#[allow(clippy::module_name_repetitions)]
pub struct PushButton;

#[derive(Component)]
#[allow(clippy::module_name_repetitions)]
pub struct ButtonSensor;

// #[derive(PartialEq)]
// pub enum ButtonState {
//     Idle,
//     Pushed,
//     Pushing,
//     Releasing,
// }

pub fn set_up(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // -- Doors --

    let button = asset_server.load("textures/level_one/button-press.png");
    let button_atlas = TextureAtlasLayout::from_grid(UVec2::new(8, 7), 1, 7, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);

    commands
        .spawn((
            SpriteBundle {
                texture: button,
                transform: Transform {
                    translation: BUTTON_POSITION.into(),
                    scale: LEVEL_SCALE.into(),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: button_atlas_handle,
                index: 0,
            },
            Name::new("OneWay Button"),
            PushButton,
            Location::Level1000,
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(1., 3.5),
                Transform::from_translation(BUTTON_HITBOX_X_OFFSET.into()),
                Name::new("Button Hitbox"),
            ));
        });
}
