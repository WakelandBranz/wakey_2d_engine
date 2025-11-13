pub mod app;
pub mod context;
pub mod game;

pub use context::Engine;
pub use game::Game;
use winit::event_loop::EventLoop;

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
