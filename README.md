# Asynchronous Train Simulator [![Rust](https://github.com/stormogulen/Railway/actions/workflows/rust.yml/badge.svg)](https://github.com/stormogulen/Railway/actions/workflows/rust.yml)


This Rust project simulates the movement of a train on a track consisting of various sections, such as straight paths, curves, obstacles, and signals. The train's movement is asynchronous, and the project demonstrates the usage of `tokio` for asynchronous tasks and error handling.

## Features

- Simulates train movement on a track with different sections.
- Demonstrates asynchronous programming using `tokio`.
- Includes error handling for various track conditions.

## How to Use
Build and run the project using Cargo, the Rust package manager:

```cargo run```

## The train simulation will start, and the output will indicate the train's progress and any encountered errors.
Track Sections

- Straight: Represents a straight section of track.
- Curve: Represents a curved section of track.
- Obstacle: Represents an obstacle on the track.
- Signal: Represents a signal on the track.

## Project Structure

- src/main.rs: Contains the main code for running the train simulation.
- src/tests.rs: Contains unit tests for the project.

## Dependencies

The project uses the following Rust crates:

- tokio: Asynchronous runtime for Rust.
- serde: Serialization and deserialization library for Rust.

## Train Simulation

The train simulation follows these steps:

- The train starts at position 0 with a speed of 1.
- The train progresses through each section of the track.
- If the train encounters an obstacle or signal, it reacts accordingly.
- If the train encounters a curve, it may derail depending on its position.
- The simulation reports the train's progress and any errors encountered.
