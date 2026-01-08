use winit::event::WindowEvent;

use crate::engine::Engine;

pub trait Game: Send + 'static {
    fn init(&mut self, engine: &mut Engine);
    // Crappy implementation of systems. The user HAS to define this themself.
    // I'll make this more ECS-esque in the future.
    fn update(&mut self, engine: &mut Engine, delta_time: f32);
    // By default does nothing, but the user should definitely implement this.
    fn on_resize(&mut self, engine: &mut Engine, width: f32, height: f32);
    // User can pass unique events
    fn on_event(&mut self, _engine: &mut Engine, _event: &WindowEvent) -> bool {
        false
    }
}
