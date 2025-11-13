//! Timing utilities for tracking frame and game time

/// Tracks timing information across frames
#[derive(Clone, Copy, Debug)]
pub struct Time {
    delta_time: f32,
    elapsed_time: f32,
    frame_count: u32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            elapsed_time: 0.0,
            frame_count: 0,
        }
    }

    /// Update with the current frame's delta time
    pub(crate) fn update(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
        self.elapsed_time += delta_time;
        self.frame_count += 1;
    }

    /// Time in seconds since the last frame
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    /// Total time in seconds since the game started
    pub fn elapsed_time(&self) -> f32 {
        self.elapsed_time
    }

    /// Total number of frames since the game started
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    /// Current frames per second
    pub fn fps(&self) -> f32 {
        if self.delta_time > 0.0 {
            1.0 / self.delta_time
        } else {
            0.0
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}
