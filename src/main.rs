mod ball;
mod players;

use ball::BallPlugin;
use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use players::PlayersPlugin;

#[derive(Resource)]
pub struct Scores {
    pub player1: u32,
    pub player2: u32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".into(),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(ShapePlugin)
        .add_plugins(PlayersPlugin)
        .add_plugins(BallPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .insert_resource(Scores {
            player1: 0,
            player2: 0,
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
