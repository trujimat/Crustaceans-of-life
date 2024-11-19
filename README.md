# Crustaceans-of-Life  

**Crustaceans-of-Life** is an implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) built in Rust using the [ggez](https://github.com/ggez/ggez) game engine.  

## Features  

- **Dynamic Cell Interaction**: Modify the grid interactively using mouse clicks.  
- **Smooth Animations**: Responsive gameplay with adjustable frame rates.  
- **Customizable Grid**: Easily configure grid size and initial patterns.   

## Installation  

1. **Prerequisites**:
   - Linux Operating system, the project has not been tested on Windows
   - Rust (latest stable version). Install it from [rust-lang.org](https://www.rust-lang.org/tools/install).  
   - A C compiler (required for `ggez` dependencies)  

3. **Clone the Repository**:  
   ```bash
   git clone https://github.com/your-username/crustaceans-of-life.git
   cd crustaceans-of-life
   
4. **Build and Run**:
   ```bash
   cargo run

5. **How to play**:
   1. **Mouse Interaction**:
   Left-click on any cell to toggle its state (alive or dead).
   2. **Keyboard Interaction**:
   Press Space to start the simulation.

## About the game

The Game of Life is fascinating because of its unpredictability. There is no known algorithm that can universally determine if a given initial configuration will eventually:

- Die out (all cells become dead),
- Stabilize (reach a repeating pattern), or
- Grow infinitely.

Each starting configuration leads to emergent pattern.

