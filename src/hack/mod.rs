use bevy::{prelude::*, winit::WinitSettings};

use crate::{constants::ui::tablet::*, locations::level_one::doors::{Door, OpenDoorEvent}};

pub struct HackPlugin;

impl Plugin for HackPlugin {
    #[rustfmt::skip]
    fn build(&self, app: &mut App) {
        app
            // OPTIMIZE: Only run the app when there is user input. This will significantly reduce CPU/GPU use.
            .insert_resource(WinitSettings::game())
            
            .add_startup_system(setup_tablet_button)
            .add_system(button_system)
            ;
    }
}

#[derive(Component)]
pub struct Hackable;

pub fn setup_tablet_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 'Hack'/Open the ALT_DOOR
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: UiRect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    position: UiRect {
                        right: Val::Percent(-39.),
                        top: Val::Percent(30.),
                        ..default()
                    },
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            Name::new("Open Alt Door"),
            // EndOfTurnButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "HACK",
                TextStyle {
                    font: asset_server.load("fonts/dpcomic.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

// pub fn hack_door(
//     mut open_doors_alt_event: EventWriter<OpenAltDoorsEvent>,
// ) {
// }

/// # Note
/// 
/// Spamming should not work (cause of the timer being only 0.1s)
/// 
/// TODO: feature - limit the access to tablet's features when using one
/// Can't hack while MindCtrl -------------------^^^^^^^^
/// REFACTOR: seperate color/text management from action
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,

    mut text_query: Query<&mut Text>,

    hackable_door: Query<Entity, (With<Door>, With<Hackable>)>,
    mut open_door_event: EventWriter<OpenDoorEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                // hack every hackable door
                for door in hackable_door.iter() {
                    open_door_event.send(OpenDoorEvent(door));
                }

                text.sections[0].value = String::from("HOCK");
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = String::from("HACK");
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = String::from("HACK");
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
