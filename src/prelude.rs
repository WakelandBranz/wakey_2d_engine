// Used Claude to generate this. I'll update it as I go, but in the interest of time this is what I've got.

//! # 2D Engine Prelude
//!
//! This module provides the essential types and traits needed to create a game with Wakey.
//!
//! ## Quick Start
//!
//! ```no_run
//! use wakey_2d_engine::prelude::*;
//!
//! struct MyGame {
//!     // Your game state here
//! }
//!
//! impl Game for MyGame {
//!     fn init(&mut self, engine: &mut Engine) {
//!         // Called once when the game starts
//!     }
//!
//!     fn update(&mut self, engine: &mut Engine, delta_time: f32) {
//!         // Called every frame
//!         // delta_time is the time since the last frame in seconds
//!         let renderer = engine.renderer_mut();
//!         renderer.queue_rectangle(10.0, 10.0, 100.0, 100.0, [1.0, 0.0, 0.0, 1.0]);
//!     }
//!
//!     fn on_event(&mut self, engine: &mut Engine, event: &winit::event::WindowEvent) -> bool {
//!         // Handle window events (keyboard, mouse, etc.)
//!         // Return true if the event was handled
//!         false
//!     }
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     App::run(
//!         EngineConfig::new()
//!             .with_title("My Game")
//!             .with_size(800, 600),
//!         MyGame {},
//!     )?;
//!     Ok(())
//! }
//! ```
//!
//! ## Key Types
//!
//! - **`App`**: The entry point. Call `App::run()` to start your game.
//! - **`Engine`**: The runtime context available in your game code.
//!   - Access the renderer: `engine.renderer_mut()`
//!   - Access the ECS world: `engine.world_mut()`
//! - **`Game`**: The trait you implement to define your game logic.
//! - **`EngineConfig`**: Configuration for the engine (window title, size, etc.).

pub use crate::engine::{App, Engine, EngineConfig, Game};
