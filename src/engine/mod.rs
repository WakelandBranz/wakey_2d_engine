pub mod app;
pub mod game;

use bevy_ecs::{resource::Resource, world::World};
pub use game::Game;
use wgpu_renderer::Renderer;
use winit::{event_loop::EventLoop, keyboard::KeyCode};

use crate::core::{input::Input, time::Time};

// Builder-style configuration struct
pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    // Add more properties as needed!!!!!
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Wakey 2D Engine".to_string(),
            window_width: 800,
            window_height: 600,
        }
    }
}

impl EngineConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.window_title = title.into();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }
}

/// Entry point for running the engine with a user-defined game
pub struct App;

impl App {
    /// Run the engine with a user-defined game
    pub fn run<G: game::Game>(
        config: EngineConfig,
        game: G,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let mut app = app::EngineApp::new(config, game);
        event_loop.run_app(&mut app)?;
        Ok(())
    }
}

// Public facing engine
pub struct Engine {
    pub(crate) renderer: Renderer,
    pub(crate) world: World,
    pub(crate) input: Input<KeyCode>,
    pub(crate) time: Time,
}

impl Engine {
    // TODO: Make Result return type
    pub(crate) fn init(&mut self, renderer: Renderer, input: Input<KeyCode>, time: Time) {
        self.world.insert_resource(Renderer::new);
    }
    
    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn input(&self) -> &Input<KeyCode> {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input<KeyCode> {
        &mut self.input
    }

    pub fn time(&self) -> Time {
        self.time
    }

    pub fn time_mut(&mut self) -> &mut Time {
        &mut self.time
    }
}
