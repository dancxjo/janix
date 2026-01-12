# Bloom (Demo Compositor)

Bloom is an app that renders into a shared Bytespace. It never claims devices and never maps scanout memory.

## Responsibilities

- Connect to Blossom over the Display Protocol v0.
- Request display info and the compositor bytespace id.
- Map the compositor bytespace locally for writing.
- Render a deterministic animation and send PRESENT messages.

## Future direction

Bloom will evolve into a real compositor (surface management, damage tracking, composition). For v0 it only proves the pipeline and boundaries.
