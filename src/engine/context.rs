use bevy_ecs::world::World;
use wgpu_renderer::Renderer;
use winit::keyboard::KeyCode;

use crate::core::{input::Input, time::Time};

// Public facing engine
pub struct Engine {
    pub(crate) renderer: Renderer,
    pub(crate) world: World,
    pub(crate) input: Input<KeyCode>,
    pub(crate) time: Time,
}

impl Engine {
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

    pub(crate) fn time_mut(&mut self) -> &mut Time {
        &mut self.time
    }
}
