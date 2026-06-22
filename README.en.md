# MazeCraze

[中文](README.md) | [English](README.en.md)

> A TUI interactive maze generator and solver with real-time algorithm animation.

## Features

- **3 Maze Generation Algorithms**: Recursive Backtracker, Randomized Prim, Randomized Kruskal
- **4 Maze Solving Algorithms**: BFS (shortest path), DFS, A\* (heuristic), Wall Follower
- **Real-time Animation**: Watch every step of generation and solving with play/pause/step controls
- **Dual Rendering Modes**: Unicode Box Drawing (beautiful) and ASCII (compatible)
- **Interactive TUI**: Built with `ratatui` for a smooth terminal experience
- **CLI Batch Mode**: Generate and solve mazes from the command line
- **Export**: Save mazes as text files
- **Challenge Mode**: Move through the maze with arrow keys / WASD and save local best records

## Quick Start

```bash
# Clone and enter the project
cd mazecraze

# Build
cargo build --release

# Run in TUI mode (default)
cargo run

# CLI batch mode
cargo run -- --generate backtracker --solve astar --size 31x31 --export maze.txt
```

## Controls

| Key | Action |
|-----|--------|
| `g` | Generate a new maze |
| `s` | Solve the current maze |
| `Space` | Play / Pause animation |
| `→` / `l` | Step forward |
| `←` / `h` | Step backward |
| `+` / `]` | Increase playback speed |
| `r` | Restart current animation |
| `p` | Start challenge mode |
| `↑↓←→` / `WASD` | Move the player in challenge mode |
| `h` | Show help |
| `q` / `Esc` | Quit / Return to menu |

## Project Structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Public API
├── core/                # Grid, Cell, Point, Direction
├── generator/           # Maze generation algorithms + trait
├── solver/              # Maze solving algorithms + trait
├── renderer/            # Unicode & ASCII renderers
├── animation/           # Frame recorder & player
├── tui/                 # Terminal UI (ratatui)
├── export/              # Text export
└── cli/                 # Command-line arguments
tests/
├── grid_tests.rs
├── generator_tests.rs
├── solver_tests.rs
├── integration_tests.rs
└── challenge_tests.rs
frontend/                # React + TypeScript Web frontend
```

## Rust Features Demonstrated

- **Ownership & Borrowing**: Strict separation of mutable generation and immutable rendering
- **Trait System**: `MazeGenerator`, `MazeSolver`, `Renderer` traits for pluggable algorithms
- **Generics**: `AnimationRecorder` works with any algorithm
- **Error Handling**: Custom `GridError` with `Result` propagation, minimal `unwrap`
- **Pattern Matching**: Exhaustive `match` on `Cell`, `Direction`, `AppState`
- **Testing**: Unit tests + integration tests covering all algorithm combinations
- **Engineering**: `cargo fmt`, `cargo clippy`, modular workspace

## Web Frontend

The project also includes a React + TypeScript + Vite web frontend in `frontend/`.
It renders mazes with Canvas and supports generation, solving, and challenge features similar to the TUI.
See `frontend/README.en.md` for details.

## License

MIT
