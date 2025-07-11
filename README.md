# Computer Graphics Lab 1 - Polygon Rendering

This project is a Rust implementation of basic computer graphics algorithms for rendering polygons, including line drawing and polygon filling.

## Requirements

- Rust programming language (latest stable version recommended)
- Cargo (Rust's package manager)
- A Rust-compatible development environment

## How to Run

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd lab1_graficas
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the program:
   ```bash
   cargo run --release
   ```

## Output

The program will generate an output file named `out.bmp` in the project's root directory. This file contains the rendered image with all the polygons drawn according to the implementation in the code.

## Project Structure

- `src/main.rs`: Main application entry point
- `src/framebuffer.rs`: Handles the frame buffer for pixel manipulation
- `src/line.rs`: Implements line drawing algorithms
- `src/polygon.rs`: Defines polygon structures and operations
- `src/polygon_fill.rs`: Implements polygon filling algorithms

## Notes

- The program is configured to render a set of predefined polygons with different colors and fill patterns.
- The output image is saved in BMP format with the name `out.bmp`.
- Make sure you have write permissions in the project directory to save the output file.