use bevy_ecs::component::Component;
use wakey_2d_engine::{
    core::world::{Bounds, Position, Renderable, Size, Velocity},
    prelude::*,
};
use winit::keyboard::KeyCode;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 120.0;
const PADDLE_SPEED: f32 = 400.0; // pixels per second
const BALL_SIZE: f32 = 10.0;
const BALL_SPEED: f32 = 400.0; // pixels per second

// Marker components for entity types
#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct AIPaddle;

#[derive(Component)]
struct Ball;

struct Pong;

impl Game for Pong {
    fn init(&mut self, engine: &mut Engine) {
        let world = engine.world_mut();

        // Create player paddle (left side)
        world.spawn((
            PlayerPaddle,
            Position::new(20.0, (WINDOW_HEIGHT as f32 - PADDLE_HEIGHT) / 2.0),
            Size::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            Renderable::white(),
            Velocity::new(0.0, 0.0),
            Bounds::from_position_and_size(
                Position::new(20.0, (WINDOW_HEIGHT as f32 - PADDLE_HEIGHT) / 2.0),
                Size::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            ),
        ));

        // Create AI paddle (right side)
        world.spawn((
            AIPaddle,
            Position::new(
                WINDOW_WIDTH as f32 - PADDLE_WIDTH - 20.0,
                (WINDOW_HEIGHT as f32 - PADDLE_HEIGHT) / 2.0,
            ),
            Size::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            Renderable::white(),
            Velocity::new(0.0, 0.0),
            Bounds::from_position_and_size(
                Position::new(
                    WINDOW_WIDTH as f32 - PADDLE_WIDTH - 20.0,
                    (WINDOW_HEIGHT as f32 - PADDLE_HEIGHT) / 2.0,
                ),
                Size::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            ),
        ));

        // Create ball
        world.spawn((
            Ball,
            Position::new(
                (WINDOW_WIDTH as f32 - BALL_SIZE) / 2.0,
                (WINDOW_HEIGHT as f32 - BALL_SIZE) / 2.0,
            ),
            Size::new(BALL_SIZE, BALL_SIZE),
            Renderable::white(),
            Velocity::new(BALL_SPEED, BALL_SPEED),
            Bounds::from_position_and_size(
                Position::new(
                    (WINDOW_WIDTH as f32 - BALL_SIZE) / 2.0,
                    (WINDOW_HEIGHT as f32 - BALL_SIZE) / 2.0,
                ),
                Size::new(BALL_SIZE, BALL_SIZE),
            ),
        ));
    }

    fn update(&mut self, engine: &mut Engine, delta_time: f32) {
        // Run Pong-specific systems
        player_paddle_system(engine, delta_time);
        ball_physics_system(engine, delta_time);
        ball_paddle_collision_system(engine);
        ai_paddle_system(engine, delta_time);
    }

    fn on_event(&mut self, _engine: &mut Engine, _event: &winit::event::WindowEvent) -> bool {
        false
    }
}

// ===== ECS Systems =====

fn player_paddle_system(engine: &mut Engine, delta_time: f32) {
    let input = engine.input().clone();
    let world = engine.world_mut();

    let mut query = world.query::<(&mut Position, &mut Bounds, &PlayerPaddle)>();

    // Collect movement updates first
    let updates: Vec<f32> = query
        .iter(world)
        .map(|(pos, _, _)| {
            let paddle_velocity =
                if input.is_pressed(KeyCode::ArrowUp) || input.is_pressed(KeyCode::KeyW) {
                    -PADDLE_SPEED
                } else if input.is_pressed(KeyCode::ArrowDown) || input.is_pressed(KeyCode::KeyS) {
                    PADDLE_SPEED
                } else {
                    0.0
                };

            (pos.y + paddle_velocity * delta_time)
                .max(0.0)
                .min(WINDOW_HEIGHT as f32 - PADDLE_HEIGHT)
        })
        .collect();

    // Apply updates
    let mut query = world.query::<(&mut Position, &mut Bounds, &PlayerPaddle)>();
    let mut iter = query.iter_mut(world);
    for new_y in updates {
        if let Some((mut pos, mut bounds, _)) = iter.next() {
            pos.y = new_y;
            bounds.min_y = new_y;
            bounds.max_y = new_y + PADDLE_HEIGHT;
        }
    }
}

fn ball_physics_system(engine: &mut Engine, delta_time: f32) {
    let world = engine.world_mut();

    // Update ball position
    let mut query = world.query::<(&mut Position, &Velocity, &Ball)>();
    let updates: Vec<(f32, f32)> = query
        .iter(world)
        .map(|(pos, vel, _)| (pos.x + vel.x * delta_time, pos.y + vel.y * delta_time))
        .collect();

    let mut query = world.query::<(&mut Position, &Velocity, &Ball)>();
    let mut iter = query.iter_mut(world);
    for (new_x, new_y) in updates {
        if let Some((mut pos, _, _)) = iter.next() {
            pos.x = new_x;
            pos.y = new_y;
        }
    }

    // Handle wall collisions
    let mut query = world.query::<(&mut Position, &mut Velocity, &Ball)>();
    let bounce_updates: Vec<bool> = query
        .iter(world)
        .map(|(pos, _, _)| pos.y <= 0.0 || pos.y + BALL_SIZE >= WINDOW_HEIGHT as f32)
        .collect();

    let mut query = world.query::<(&mut Position, &mut Velocity, &Ball)>();
    let mut iter = query.iter_mut(world);
    for should_bounce in bounce_updates {
        if let Some((mut pos, mut vel, _)) = iter.next() {
            if should_bounce {
                vel.y = -vel.y;
                pos.y = pos.y.max(0.0).min(WINDOW_HEIGHT as f32 - BALL_SIZE);
            }
        }
    }

    // Update ball bounds
    let mut query = world.query::<(&Position, &mut Bounds, &Ball)>();
    let updates: Vec<(f32, f32)> = query
        .iter(world)
        .map(|(pos, _, _)| (pos.x, pos.y))
        .collect();

    let mut query = world.query::<(&Position, &mut Bounds, &Ball)>();
    let mut iter = query.iter_mut(world);
    for (x, y) in updates {
        if let Some((_, mut bounds, _)) = iter.next() {
            bounds.min_x = x;
            bounds.max_x = x + BALL_SIZE;
            bounds.min_y = y;
            bounds.max_y = y + BALL_SIZE;
        }
    }

    // Reset ball if out of bounds
    let mut query = world.query::<(&mut Position, &mut Velocity, &Ball)>();
    let mut iter = query.iter_mut(world);
    if let Some((mut pos, mut vel, _)) = iter.next() {
        if pos.x < 0.0 || pos.x > WINDOW_WIDTH as f32 {
            pos.x = (WINDOW_WIDTH as f32 - BALL_SIZE) / 2.0;
            pos.y = (WINDOW_HEIGHT as f32 - BALL_SIZE) / 2.0;
            vel.x = BALL_SPEED;
            vel.y = BALL_SPEED;
        }
    }
}

fn ball_paddle_collision_system(engine: &mut Engine) {
    let world = engine.world_mut();

    // Get ball bounds
    let mut ball_query = world.query::<(&Bounds, &mut Velocity, &Ball)>();
    let ball_data: Vec<(Bounds, bool, bool)> = ball_query
        .iter(world)
        .map(|(bounds, vel, _)| {
            (
                *bounds,
                vel.x < 0.0, // moving left
                vel.x > 0.0, // moving right
            )
        })
        .collect();

    for (ball_bounds, moving_left, moving_right) in ball_data {
        // Check collision with player paddle
        if moving_left {
            let mut paddle_query = world.query::<(&Bounds, &PlayerPaddle)>();
            for (paddle_bounds, _) in paddle_query.iter(world) {
                if ball_bounds.intersects(paddle_bounds) {
                    let mut vel_query = world.query::<(&mut Velocity, &Ball)>();
                    for (mut vel, _) in vel_query.iter_mut(world) {
                        vel.x = -vel.x;
                        break; // Only hit one ball
                    }
                    break;
                }
            }
        }

        // Check collision with AI paddle
        if moving_right {
            let mut paddle_query = world.query::<(&Bounds, &AIPaddle)>();
            for (paddle_bounds, _) in paddle_query.iter(world) {
                if ball_bounds.intersects(paddle_bounds) {
                    let mut vel_query = world.query::<(&mut Velocity, &Ball)>();
                    for (mut vel, _) in vel_query.iter_mut(world) {
                        vel.x = -vel.x;
                        break; // Only hit one ball
                    }
                    break;
                }
            }
        }
    }
}

fn ai_paddle_system(engine: &mut Engine, delta_time: f32) {
    let world = engine.world_mut();

    // Get ball position
    let mut ball_query = world.query::<(&Position, &Ball)>();
    let ball_center = if let Some((pos, _)) = ball_query.iter(world).next() {
        pos.y + BALL_SIZE / 2.0
    } else {
        return;
    };

    // Update AI paddle
    let mut ai_query = world.query::<(&mut Position, &mut Bounds, &AIPaddle)>();
    let ai_updates: Vec<f32> = ai_query
        .iter(world)
        .map(|(pos, _, _)| {
            let paddle_center = pos.y + PADDLE_HEIGHT / 2.0;

            // Update the speed based on difficulty.
            // 0.7 is slightly slower than the player.
            let ai_speed = PADDLE_SPEED * 0.7;

            if paddle_center < ball_center - 10.0 {
                (paddle_center + ai_speed * delta_time).min(WINDOW_HEIGHT as f32 - PADDLE_HEIGHT)
                    - PADDLE_HEIGHT / 2.0
            } else if paddle_center > ball_center + 10.0 {
                (paddle_center - ai_speed * delta_time).max(0.0) - PADDLE_HEIGHT / 2.0
            } else {
                pos.y
            }
        })
        .collect();

    let mut ai_query = world.query::<(&mut Position, &mut Bounds, &AIPaddle)>();
    let mut iter = ai_query.iter_mut(world);
    for new_y in ai_updates {
        if let Some((mut pos, mut bounds, _)) = iter.next() {
            pos.y = new_y;
            bounds.min_y = new_y;
            bounds.max_y = new_y + PADDLE_HEIGHT;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::run(
        EngineConfig::new()
            .with_title("Pong")
            .with_size(WINDOW_WIDTH, WINDOW_HEIGHT),
        Pong,
    )?;
    Ok(())
}
