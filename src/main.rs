#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::type_complexity, clippy::too_many_arguments, clippy::pedantic)]
// #![warn(missing_docs)]

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

use characters::CharactersPlugin;
use constants::{RESOLUTION, TILE_SIZE};
use debug::DebugPlugin;
use locations::LocationsPlugin;
use spritesheet::CatSpritePlugin;
use tablet::hack::HackPlugin;
use tablet::mind_control::MindControlPlugin;

pub mod characters;
pub mod collisions;
pub mod constants;
mod debug;
pub mod locations;
mod spritesheet;
pub mod tablet;

#[rustfmt::skip]
fn main() {
    let height = 1080.;

    let mut app = App::new();
    app
        // Color::TEAL / AZURE
        .insert_resource(ClearColor(Color::TEAL))
        .insert_resource(Msaa::Off)
        // v-- Hitbox --v
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })

        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(height * RESOLUTION, height),
                        title: "CatBeDoingTheLaundry".to_string(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        // .add_plugin(TweeningPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(CatSpritePlugin)
        .add_plugin(HackPlugin)
        .add_plugin(LocationsPlugin)
        .add_plugin(MindControlPlugin)
        .add_plugin(CharactersPlugin)
        .add_startup_system(spawn_camera);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.1;

    // vv-- Flip the left and right --vv
    // camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}
