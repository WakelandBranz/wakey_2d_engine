use bevy_ecs::component::Component;
use wakey_2d_engine::{
    core::components::{Bounds, Position, Renderable, Size, Velocity},
    prelude::*,
};
use winit::keyboard::KeyCode;

const FONT_BYTES: &[u8] = include_bytes!("../res/fonts/PressStart2P-Regular.ttf");

// Crappy defaults. I want to make this easier for users to define in other ways.
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 120.0;
const PADDLE_SPEED: f32 = 400.0; // pixels per second
const AI_SPEED_OFFSET: f32 = 0.7;
const BALL_SIZE: f32 = 10.0;
const BALL_SPEED: f32 = 400.0; // pixels per second
const ORIGINAL_HEIGHT: f32 = 600.0;

// Score events
#[derive(Debug, Clone, Copy)]
enum ScoreEvent {
    PlayerScored,
    AIScored,
    NoScore,
}

// Marker components for entity types
#[derive(Component)]
struct PlayerPaddle;

#[derive(Component)]
struct AIPaddle;

#[derive(Component)]
struct Ball;

struct Pong {
    player_score: u32,
    ai_score: u32,
}

impl Game for Pong {
    fn init(&mut self, engine: &mut Engine) {
        let screen_dimensions = (engine.renderer().width(), engine.renderer().height());
        let world = engine.world_mut();
        
        let fps_text = engine.renderer_mut().create_cached_text(, );
        
        world.insert_resource(value);
        
        //let video_game_awesome_font_handle = renderer.load_ttf_font_from_bytes(FONT_BYTES.into()).unwrap();
        //renderer.set_default_font(video_game_awesome_font_handle);

        // Create player paddle (left side)
        world.spawn((
            PlayerPaddle,
            Position::new(20.0, (screen_dimensions.1 as f32 - PADDLE_HEIGHT) / 2.0),
            Size::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            Renderable::white(),
            Velocity::new(0.0, 0.0),
            Bounds::from_position_and_size(
                Position::new(20.0, (screen_dimensions.1 as f32 - PADDLE_HEIGHT) / 2.0),
                Size::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            ),
        ));

        // Create AI paddle (right side)
        world.spawn((
            AIPaddle,
            Position::new(
                screen_dimensions.0 as f32 - PADDLE_WIDTH - 20.0,
                (screen_dimensions.1 as f32 - PADDLE_HEIGHT) / 2.0,
            ),
            Size::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            Renderable::red(),
            Velocity::new(PADDLE_SPEED * AI_SPEED_OFFSET, PADDLE_SPEED * AI_SPEED_OFFSET),
            Bounds::from_position_and_size(
                Position::new(
                    screen_dimensions.0 as f32 - PADDLE_WIDTH - 20.0,
                    (screen_dimensions.1 as f32 - PADDLE_HEIGHT) / 2.0,
                ),
                Size::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            ),
        ));

        // Create ball
        world.spawn((
            Ball,
            Position::new(
                (screen_dimensions.0 as f32 - BALL_SIZE) / 2.0,
                (screen_dimensions.1 as f32 - BALL_SIZE) / 2.0,
            ),
            Size::new(BALL_SIZE, BALL_SIZE),
            Renderable::white(),
            Velocity::new(BALL_SPEED, BALL_SPEED),
            Bounds::from_position_and_size(
                Position::new(
                    (screen_dimensions.0 as f32 - BALL_SIZE) / 2.0,
                    (screen_dimensions.1 as f32 - BALL_SIZE) / 2.0,
                ),
                Size::new(BALL_SIZE, BALL_SIZE),
            ),
        ));
    }

    fn update(&mut self, engine: &mut Engine, delta_time: f32) {
        // Run Pong-specific systems
        player_paddle_system(engine, delta_time);
        let score_event = ball_physics_system(engine, delta_time);
        ball_paddle_collision_system(engine);
        ai_paddle_system(engine, delta_time);

        // Update score if ball went out of bounds
        match score_event {
            ScoreEvent::PlayerScored => self.player_score += 1,
            ScoreEvent::AIScored => self.ai_score += 1,
            ScoreEvent::NoScore => {}
        }

        // Render UI
        let fps = engine.time().fps();
        let screen_width = engine.renderer().width();

        engine.renderer_mut().queue_text(
            &format!("FPS: {:.1}", fps),
            (10.0, 10.0),         // position
            10.0,                 // font size
            [1.0, 1.0, 1.0, 1.0], // white color
        );

        // Render scores
        engine.renderer_mut().queue_text(
            &format!("Player: {}", self.player_score),
            (10.0, 30.0),
            16.0,
            [1.0, 1.0, 1.0, 1.0],
        );

        engine.renderer_mut().queue_text(
            &format!("AI: {}", self.ai_score),
            (screen_width - 150.0, 30.0),
            16.0,
            [1.0, 1.0, 1.0, 1.0],
        );
    }

    fn on_resize(&mut self, engine: &mut Engine, width: f32, height: f32) {
        let world = engine.world_mut();

        let scale_factor = height / ORIGINAL_HEIGHT;

        // Scale paddle dimensions
        let scaled_paddle_height = PADDLE_HEIGHT * scale_factor;
        let scaled_paddle_width = PADDLE_WIDTH * scale_factor;

        // Update AI paddle position and size
        let mut ai_query = world.query::<(&mut Velocity, &mut Position, &mut Bounds, &mut Size, &AIPaddle)>();
        for (mut vel, mut pos, mut bounds, mut size, _) in ai_query.iter_mut(world) {
            pos.x = width - scaled_paddle_width - 20.0;
            size.width = scaled_paddle_width;
            size.height = scaled_paddle_height;

            bounds.min_x = pos.x;
            bounds.max_x = pos.x + scaled_paddle_width;
            bounds.min_y = pos.y;
            bounds.max_y = pos.y + scaled_paddle_height;

            vel.y = vel.y.signum() * PADDLE_SPEED * AI_SPEED_OFFSET * scale_factor;
        }

        // Update player paddle size (position stays at x=20)
        let mut player_query =
            world.query::<(&mut Velocity, &mut Position, &mut Bounds, &mut Size, &PlayerPaddle)>();
        for (mut vel, pos, mut bounds, mut size, _) in player_query.iter_mut(world) {
            size.width = scaled_paddle_width;
            size.height = scaled_paddle_height;

            bounds.min_x = pos.x;
            bounds.max_x = pos.x + scaled_paddle_width;
            bounds.min_y = pos.y;
            bounds.max_y = pos.y + scaled_paddle_height;

            vel.y = vel.y.signum() * PADDLE_SPEED * scale_factor;
        }

        // Update ball size and speed
        let mut ball_query = world.query::<(&mut Velocity, &mut Size, &Ball)>();
        for (mut vel, mut size, _) in ball_query.iter_mut(world) {
            let scaled_ball_size = BALL_SIZE * scale_factor;
            size.width = scaled_ball_size;
            size.height = scaled_ball_size;
            
            vel.x = vel.x.signum() * BALL_SPEED * scale_factor;
            vel.y = vel.y.signum() * BALL_SPEED * scale_factor;
        }
    }

    fn on_event(&mut self, engine: &mut Engine, event: &winit::event::WindowEvent) -> bool {
        false
    }
}

// ===== ECS Systems =====

fn player_paddle_system(engine: &mut Engine, delta_time: f32) {
    let input = engine.input().clone();
    let screen_dimensions = (engine.renderer().width(), engine.renderer().height());
    let world = engine.world_mut();
    let mut query = world.query::<(&mut Velocity, &mut Position, &mut Bounds, &Size, &PlayerPaddle)>();

    // Collect movement updates first
    let updates: Vec<f32> = query
        .iter(world)
        .map(|(vel, pos, _, size, _)| {
            let paddle_velocity =
                if input.is_pressed(KeyCode::ArrowUp) || input.is_pressed(KeyCode::KeyW) {
                    vel.y * -1.0
                } else if input.is_pressed(KeyCode::ArrowDown) || input.is_pressed(KeyCode::KeyS) {
                    vel.y
                } else {
                    0.0
                };

            (pos.y + paddle_velocity * delta_time)
                .max(0.0)
                .min(screen_dimensions.1 - size.height)
        })
        .collect();

    // Apply updates
    let mut query = world.query::<(&mut Position, &mut Bounds, &Size, &PlayerPaddle)>();
    let mut iter = query.iter_mut(world);
    for new_y in updates {
        if let Some((mut pos, mut bounds, size, _)) = iter.next() {
            pos.y = new_y;
            bounds.min_y = new_y;
            bounds.max_y = new_y + size.height;
        }
    }
}

fn ball_physics_system(engine: &mut Engine, delta_time: f32) -> ScoreEvent {
    let screen_dimensions = (engine.renderer().width(), engine.renderer().height());
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

    // Handle wall collisions (clamp-based, not just bounce detection)
    let mut query = world.query::<(&mut Position, &mut Velocity, &Size, &Ball)>();
    let mut iter = query.iter_mut(world);
    if let Some((mut pos, mut vel, size, _)) = iter.next() {
        // Top wall collision
        if pos.y <= 0.0 {
            pos.y = 0.0;
            vel.y = vel.y.abs(); // Bounce downward (ensure positive velocity)
        }
        // Bottom wall collision
        if pos.y + size.height >= screen_dimensions.1 {
            pos.y = screen_dimensions.1 - size.height;
            vel.y = -vel.y.abs(); // Bounce upward (ensure negative velocity)
        }
    }

    // Update ball bounds
    let mut query = world.query::<(&Position, &mut Bounds, &Size, &Ball)>();
    let updates: Vec<(f32, f32)> = query
        .iter(world)
        .map(|(pos, _, _, _)| (pos.x, pos.y))
        .collect();

    let mut query = world.query::<(&Position, &mut Bounds, &Size, &Ball)>();
    let mut iter = query.iter_mut(world);
    for (x, y) in updates {
        if let Some((_, mut bounds, size, _)) = iter.next() {
            bounds.min_x = x;
            bounds.max_x = x + size.width;
            bounds.min_y = y;
            bounds.max_y = y + size.height;
        }
    }

    // Reset ball if out of bounds and return scoring event
    let mut query = world.query::<(&mut Position, &mut Velocity, &Size, &Ball)>();
    let mut iter = query.iter_mut(world);
    if let Some((mut pos, mut vel, size, _)) = iter.next() {
        if pos.x < 0.0 {
            // Ball went past left side - AI scores
            pos.x = (screen_dimensions.0 - size.width) / 2.0;
            pos.y = (screen_dimensions.1 - size.height) / 2.0;
            vel.x = vel.x;
            vel.y = vel.y;
            return ScoreEvent::AIScored;
        } else if pos.x > screen_dimensions.0 {
            // Ball went past right side - Player scores
            pos.x = (screen_dimensions.0 - size.width) / 2.0;
            pos.y = (screen_dimensions.1 - size.height) / 2.0;
            vel.x = -vel.x;
            vel.y = -vel.y;
            return ScoreEvent::PlayerScored;
        }
    }
    ScoreEvent::NoScore
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
    let screen_dimensions = (engine.renderer().width(), engine.renderer().height());
    let world = engine.world_mut();

    // Get ball position
    let mut ball_query = world.query::<(&Position, &Size, &Ball)>();
    let ball_center = if let Some((pos, size, _)) = ball_query.iter(world).next() {
        pos.y + size.height / 2.0
    } else {
        return;
    };

    // Update AI paddle
    let mut ai_query = world.query::<(&mut Velocity, &mut Position, &mut Bounds, &Size, &AIPaddle)>();
    let ai_updates: Vec<f32> = ai_query
        .iter(world)
        .map(|(vel, pos, _, size, _)| {
            let paddle_center = pos.y + size.height / 2.0;

            let paddle_velocity = if paddle_center < ball_center - 10.0 {
                vel.y
            } else if paddle_center > ball_center + 10.0 {
                vel.y * -1.0
            } else {
                0.0
            };

            // Simple position-based clamping like player paddle
            (pos.y + paddle_velocity * delta_time)
                .max(0.0)
                .min(screen_dimensions.1 - size.height)
        })
        .collect();

    let mut ai_query = world.query::<(&mut Position, &mut Bounds, &Size, &AIPaddle)>();
    let mut iter = ai_query.iter_mut(world);
    for new_y in ai_updates {
        if let Some((mut pos, mut bounds, size, _)) = iter.next() {
            pos.y = new_y;
            bounds.min_y = new_y;
            bounds.max_y = new_y + size.height;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::run(
        EngineConfig::new().with_title("Pong").with_size(800, 600),
        Pong {
            player_score: 0,
            ai_score: 0,
        },
    )?;
    Ok(())
}
