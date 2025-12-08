# 2D Game Engine Roadmap

> Note, I used AI to write this up. This is not permanent and is moreso a way to lay out what I want to work on soon.

A custom 2D game engine built from scratch in Rust using wgpu and bevy-ECS.

## Current State

The engine currently supports:
- Entity Component System architecture (bevy-ECS)
- Text rendering
- Colored quad rendering
- Playable Pong demo
- Web deployment via WASM

## Development Roadmap

### Phase 1: Minimal Texture Support

Goal: Enable basic texture sampling to support visually polished lighting demos.

- Load PNG images into GPU textures
- Sample textures in fragment shader
- Apply textures to existing quad rendering

This is infrastructure work—keep it minimal. No atlas systems, no sprite sheets. Just enough to make a textured sprite with soft edges.

### Phase 2: 2D Dynamic Lighting

Goal: Transform the Pong demo into a visually impressive tech showcase with dynamic lighting and shadows.

Core features:
- Point light component in ECS
- 2D raycast shadow generation from light sources
- Light accumulation buffer
- Scene compositing with lighting

Stretch goals:
- Port shadow computation to GPU compute shader
- Soft shadow falloff
- Multiple light sources with different colors

Demo target: The ball acts as a moving light source. Paddles and walls cast real-time shadows. The scene has atmosphere and depth.

### Phase 3: Particle System

Goal: Add visual polish and demonstrate compute shader proficiency.

- Particle emitter component
- GPU-driven particle simulation via compute shaders
- Integration with lighting system (particles emit/receive light)
- Ball impact effects and trails

### Phase 4: Post-Processing Pipeline

Goal: Screen-space effects that demonstrate understanding of the full rendering pipeline.

- Bloom on light sources
- Color grading
- Screen-space effects (vignette, chromatic aberration)

### Future Considerations

- Profiler/debug overlay showing frame times, draw calls, GPU memory
- Potential 3D renderer exploration (deferred shading, PBR)

## Technical Writeups

Each major feature should include a short technical breakdown covering:
- The problem being solved
- Implementation approach and alternatives considered
- Tradeoffs made and why
- Performance characteristics.