use bevy::{prelude::*, window::WindowResized};
use bevy_prototype_lyon::prelude::*;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            .add_systems(Update, controls)
            .add_systems(Update, handle_resize)
            .register_type::<Player>();
    }
}

#[derive(Component)]
pub struct Players;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub id: u16,
    pub speed: f32,
}

fn spawn_players(mut commands: Commands, window: Query<&Window>) {
    commands
        .spawn((SpatialBundle::default(), Players, Name::new("Players")))
        .with_children(|commands| {
            let window = window.single();
            let width = window.resolution.width();
            let player_pos = (width / 2.0) * 0.90;

            let player_length = 200.0;
            let player_shape = shapes::Rectangle {
                origin: shapes::RectangleOrigin::Center,
                extents: Vec2::new(15.0, player_length),
            };

            // Player 1
            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&player_shape),
                    transform: Transform::from_translation(Vec3::new(-player_pos, 0., 0.)).into(),
                    ..default()
                },
                Fill::color(Color::WHITE),
                Player {
                    id: 1,
                    speed: 450.0,
                },
                Name::new("Player 1"),
            ));

            // Player 2
            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&player_shape),
                    transform: Transform::from_translation(Vec3::new(player_pos, 0., 0.)).into(),
                    ..default()
                },
                Fill::color(Color::WHITE),
                Player {
                    id: 2,
                    speed: 450.0,
                },
                Name::new("Player 2"),
            ));
        });
}

fn controls(
    mut players: Query<(&mut Transform, &Player)>,
    window: Query<&Window>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let window = window.single();
    let height = window.resolution.height();
    let bound_y = (height / 2.0) - 100.0;
    for (mut transform, player) in &mut players {
        let movement_amount = player.speed * time.delta_seconds();
        if player.id == 1 {
            if input.pressed(KeyCode::W) && transform.translation.y <= bound_y {
                transform.translation.y += movement_amount;
            }

            if input.pressed(KeyCode::S) && transform.translation.y >= -bound_y {
                transform.translation.y -= movement_amount;
            }
        } else if player.id == 2 {
            if input.pressed(KeyCode::Up) && transform.translation.y <= bound_y {
                transform.translation.y += movement_amount;
            }

            if input.pressed(KeyCode::Down) && transform.translation.y >= -bound_y {
                transform.translation.y -= movement_amount;
            }
        }
    }
}

fn handle_resize(
    mut players: Query<(&mut Transform, &Player)>,
    mut resize_event: EventReader<WindowResized>,
) {
    for e in resize_event.iter() {
        let player_pos = (e.width / 2.0) * 0.90;
        for (mut transform, player) in &mut players {
            if player.id == 1 {
                transform.translation.x = -player_pos;
            }

            if player.id == 2 {
                transform.translation.x = player_pos;
            }
        }
    }
}
