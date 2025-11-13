use winit::event::WindowEvent;

use crate::engine::context::Engine;

pub trait Game: Send + 'static {
    fn init(&mut self, engine: &mut Engine);
    // Crappy implementation of systems. The use HAS to define this themself.
    // I'll make this more ECS-esque in the future.
    fn update(&mut self, engine: &mut Engine, delta_time: f32);
    fn on_event(&mut self, _engine: &mut Engine, _event: &WindowEvent) -> bool {
        false
    }
}
