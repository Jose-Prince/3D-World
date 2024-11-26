# No UVG's Sky

**No UVG's Sky** is a Rust-based 3D space exploration project where players can navigate a procedurally rendered solar system. The system includes a spaceship, one star, and five planets, all rendered with unique visual effects. Players can explore the system using various gameplay features such as collision avoidance, autopilot, and dynamic camera perspectives like a bird's-eye view.

---

## Features

- **Real-time Rendering**: Renders a spaceship, celestial bodies (star and planets), and a dynamic skybox.
- **Bird's-eye View**: Allows players to visualize the entire system from above.
- **Collision Avoidance**: Warns players of imminent collisions and automatically evades when necessary.
- **Barrel Roll Maneuver**: Perform barrel rolls to navigate or evade obstacles.
- **Minimap Integration**: Displays nearby celestial bodies relative to the ship.
- **Procedural Noise Generation**: Different noise algorithms for unique planet surfaces and star visuals.
- **Soundtrack**: Dynamic background music changes between menu and gameplay.

---

## Controls

| Key             | Action                              |
|------------------|-------------------------------------|
| `W` / `A` / `S` / `D` | Move the spaceship in respective directions. |
| `Enter`         | Start the game.                     |
| `Escape`        | Exit the game.                      |
| `M`             | Toggle alternate rendering mode.    |

---

## Technologies Used

- **Rust**: Core programming language.
- **`minifb`**: Window management and input handling.
- **`nalgebra_glm`**: Linear algebra and 3D transformations.
- **`FastNoiseLite`**: Procedural noise generation for realistic planet textures.
- **Custom Graphics Pipeline**: Built from scratch for rendering, collision detection, and shading.

---

## Project Structure

The project is organized into multiple modules for clarity and reusability:

- **`main.rs`**: Entry point and game loop.
- **`framebuffer`**: Handles pixel-level rendering.
- **`shader`**: Contains custom shaders for lighting and effects.
- **`camera`**: Manages camera transformations and perspective.
- **`skybox`**: Renders the background sky.
- **`minimap`**: Displays a bird's-eye view of the system.
- **`autopilot`**: Implements automatic collision evasion.
- **`audioPlayer`**: Handles background music playback.
- **`collisionWarning`**: Alerts for imminent collisions.

---

## Installation

1. **Install Rust**: Ensure Rust is installed. You can download it from [Rust's official website](https://www.rust-lang.org/).
2. **Clone this repository**:
   ```bash
   git clone https://github.com/yourusername/NoUVGsSky.git
   cd NoUVGsSky
   ```
3. **Build and Run**:
   ```bash
   cargo run
   ```

---

## Gameplay Overview

- **Exploration**: Use the spaceship to explore the solar system.
- **Dynamic Warning System**: Alerts for gravitational fields of nearby celestial bodies.
- **Autopilot Mode**: Activates automatically to evade dangerous zones.

---

## Known Issues

- **High CPU Usage**: Extended gameplay may result in higher CPU usage.
- **Visual Artifacts**: Minor glitches during rapid movements.

---

## Future Improvements

- Add more celestial bodies with diverse noise algorithms.
- Introduce interactive gameplay elements such as docking or mining.
- Optimize rendering for better performance.

---

## License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.

---

## Demo



https://github.com/user-attachments/assets/88dec44f-7320-4850-bf27-22f0d9d407a7



---

Feel free to reach out with any questions, suggestions, or contributions! 🚀
