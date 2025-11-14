use std::{sync::Arc, time::Instant};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

use crate::engine::{EngineConfig, context::Engine, game::Game};

// Internal application handler
pub struct EngineApp<G: Game> {
    engine: Option<Engine>,
    window: Option<Arc<Window>>,
    config: EngineConfig,
    game: G,
    last_frame_time: Option<Instant>,
    initialized: bool,
}

impl<G: Game> EngineApp<G> {
    pub(crate) fn new(config: EngineConfig, game: G) -> Self {
        Self {
            engine: None,
            window: None,
            config,
            game,
            last_frame_time: None,
            initialized: false,
        }
    }
}

// Generic winit stuff. All of this is required for winit.
impl<G: Game> ApplicationHandler for EngineApp<G> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(
                        WindowAttributes::default()
                            .with_title(&self.config.window_title)
                            .with_inner_size(winit::dpi::PhysicalSize {
                                width: self.config.window_width,
                                height: self.config.window_height,
                            }),
                    )
                    .expect("Failed to create window in EngineApp!"),
            );

            let size = window.inner_size();
            let renderer = pollster::block_on(wgpu_renderer::Renderer::new(window.clone(), size));

            window.request_redraw();
            self.window = Some(window);

            let mut engine = Engine {
                renderer,
                world: bevy_ecs::world::World::new(),
                input: crate::core::input::Input::new(),
                time: crate::core::time::Time::new(),
            };

            // Initialize the game once
            self.game.init(&mut engine);
            self.engine = Some(engine);
            self.initialized = true;
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(engine) = &mut self.engine {
                    engine.renderer_mut().resize(new_size);
                    // I need to find a way to make this automatic.
                    self.game
                        .on_resize(engine, new_size.width as f32, new_size.height as f32);
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(engine) = &mut self.engine {
                    match state {
                        ElementState::Pressed => {
                            engine.input_mut().press(code);
                        }
                        ElementState::Released => {
                            engine.input_mut().release(code);
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(engine) = &mut self.engine {
                    // Clear frame input state at the start of the frame
                    engine.input_mut().clear();

                    // Calculate delta time
                    let now = Instant::now();
                    let delta_time = if let Some(last_time) = self.last_frame_time {
                        (now - last_time).as_secs_f32()
                    } else {
                        0.016 // ~60 FPS estimate for first frame
                    };
                    self.last_frame_time = Some(now);

                    // Update time
                    engine.time_mut().update(delta_time);

                    // Call user's update logic - MAKE THIS ECS BASED IN THE FUTURE
                    self.game.update(engine, delta_time);

                    // Run built-in rendering system to draw ECS entities
                    crate::core::systems::render_system(engine);

                    // Render
                    let _ = engine.renderer_mut().render_frame();
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            other_event => {
                if let Some(engine) = &mut self.engine {
                    self.game.on_event(engine, &other_event);
                }
            }
        }
    }
}
