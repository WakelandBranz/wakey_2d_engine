use std::collections::HashSet;

/// Track keyboard and mouse input state (both types of input implement the same traits)
/// input.clear() is called every frame. This
#[derive(Clone)]
pub struct Input<T: Copy + Eq + std::hash::Hash> {
    pressed: HashSet<T>,
    just_pressed: HashSet<T>,
    just_released: HashSet<T>,
}

impl<T: Copy + Eq + std::hash::Hash> Input<T> {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn is_pressed(&self, input: T) -> bool {
        self.pressed.contains(&input)
    }

    /// Returns true if the key was pressed on the same frame as this was called.
    /// Useful for functions that should not be reran as a button is pressed for an extended
    /// period of time.
    pub fn is_just_pressed(&self, input: T) -> bool {
        self.just_pressed.contains(&input)
    }

    /// Returns true if the key was released on the same frame as this was called.
    /// Genuinely not totally sure when to use this lol
    pub fn is_just_released(&self, input: T) -> bool {
        self.just_released.contains(&input)
    }

    pub(crate) fn press(&mut self, input: T) {
        // pressed.insert() returns true only if the element was newly inserted
        if self.pressed.insert(input) {
            self.just_pressed.insert(input);
        }
    }

    pub(crate) fn release(&mut self, input: T) {
        if self.pressed.remove(&input) {
            self.just_released.insert(input);
        }
    }

    /// Call at the start of every frame
    pub(crate) fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}
