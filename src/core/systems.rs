//! Built-in systems for the engine
//!
//! Systems are functions that query and iterate over ECS entities
//! to perform operations like rendering or physics updates.

use crate::{
    core::world::{Position, Renderable, Size},
    engine::context::Engine,
};

/// Render system that queries all entities with (Position, Size, Renderable) components
/// and draws them to the renderer
pub fn render_system(engine: &mut Engine) {
    // Collect render data from the world first to avoid simultaneous mutable borrows
    let render_data: Vec<(f32, f32, f32, f32, [f32; 4])> = {
        let world = engine.world_mut();
        let mut query = world.query::<(&Position, &Size, &Renderable)>();

        query
            .iter(world)
            .map(|(position, size, renderable)| {
                (
                    position.x,
                    position.y,
                    size.width,
                    size.height,
                    renderable.color,
                )
            })
            .collect()
    };

    // Now queue the rectangles with the renderer (this is only able to do rectangles for now lol)
    let renderer = engine.renderer_mut();
    for (x, y, width, height, color) in render_data {
        renderer.queue_rectangle(x, y, width, height, color);
    }
}
