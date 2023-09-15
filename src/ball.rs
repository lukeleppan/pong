use std::time::Duration;

use bevy::{prelude::*, window::Window};
use bevy_prototype_lyon::prelude::*;

use crate::{players::Player, Scores};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(
                Update,
                (
                    start_ball,
                    ball_movement,
                    handle_roof_collision,
                    handle_goal_collision,
                    handle_player_collision,
                ),
            )
            .register_type::<Ball>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Ball {
    pub start_timer: Timer,
    pub start_velocity: Vec2,
    pub velocity: Vec2,
}

fn spawn_ball(mut commands: Commands) {
    let ball_shape = shapes::Circle {
        radius: 15.0,
        ..default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&ball_shape),
            ..default()
        },
        Fill::color(Color::WHITE),
        Ball {
            start_timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
            start_velocity: Vec2::new(500.0, 500.0),
            velocity: Vec2::new(0.0, 0.0),
        },
        Name::new("Ball"),
    ));
}

fn start_ball(mut ball_query: Query<&mut Ball>, time: Res<Time>) {
    let mut ball = ball_query.single_mut();

    ball.start_timer.tick(time.delta());

    if ball.start_timer.finished() {
        ball.velocity = ball.start_velocity;
        ball.start_timer.reset();
        ball.start_timer.pause();
    }
}

fn ball_movement(mut ball_query: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    let (mut transform, ball) = ball_query.single_mut();
    transform.translation.x += ball.velocity.x * time.delta_seconds();
    transform.translation.y += ball.velocity.y * time.delta_seconds();
}

fn handle_roof_collision(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    window: Query<&Window>,
) {
    let (transform, mut ball) = ball_query.single_mut();
    let window = window.single();
    let height = window.resolution.height();
    let bound_y = height / 2.0;

    if transform.translation.y >= bound_y - 15.0 {
        ball.velocity.y = -500.0;
    }
    if transform.translation.y <= -bound_y + 15.0 {
        ball.velocity.y = 500.0;
    }
}

fn handle_goal_collision(
    mut commands: Commands,
    ball_entity: Query<(Entity, &Transform, With<Ball>)>,
    mut scores: ResMut<Scores>,
    window: Query<&Window>,
) {
    let (entity, transform, _ball) = ball_entity.single();
    let window = window.single();
    let width = window.resolution.width();
    let bound_x = width / 2.0;

    let ball_shape = shapes::Circle {
        radius: 15.0,
        ..default()
    };

    if transform.translation.x >= bound_x {
        scores.player1 += 1;
        let starting_velocity = Vec2::new(-500.0, 500.0);

        commands.entity(entity).despawn();

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&ball_shape),
                ..default()
            },
            Fill::color(Color::WHITE),
            Ball {
                start_timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
                start_velocity: starting_velocity,
                velocity: Vec2::ZERO,
            },
            Name::new("Ball"),
        ));
    }
    if transform.translation.x <= -bound_x {
        scores.player2 += 1;
        let starting_velocity = Vec2::new(500.0, -500.0);

        commands.entity(entity).despawn();

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&ball_shape),
                ..default()
            },
            Fill::color(Color::WHITE),
            Ball {
                start_timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
                start_velocity: starting_velocity,
                velocity: Vec2::ZERO,
            },
            Name::new("Ball"),
        ));
    }
}

fn handle_player_collision(
    mut ball_query: Query<(&Transform, &mut Ball)>,
    mut players: Query<(&Transform, &Player)>,
) {
    let (ball_transform, mut ball) = ball_query.single_mut();
    let ball_x = ball_transform.translation.x;
    let ball_y = ball_transform.translation.y;

    for (player_transform, player) in &mut players {
        if player.id == 1 {
            let player_x = player_transform.translation.x + 7.5;
            let player_y = player_transform.translation.y;
            let player_y_top = player_y + 100.0;
            let player_y_bottom = player_y - 100.0;

            if ball_x - 15.0 <= player_x && ball_y < player_y_top && ball_y > player_y_bottom {
                ball.velocity.x = 500.0;
            }
        }

        if player.id == 2 {
            let player_x = player_transform.translation.x - 7.5;
            let player_y = player_transform.translation.y;
            let player_y_top = player_y + 100.0;
            let player_y_bottom = player_y - 100.0;

            if ball_x + 15.0 >= player_x && ball_y < player_y_top && ball_y > player_y_bottom {
                ball.velocity.x = -500.0;
            }
        }
    }
}
