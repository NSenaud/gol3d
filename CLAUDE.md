# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

gol3d — a 3D Conway's Game of Life written in Rust, visualized with Kiss3D. The game runs in a cubic board where cells live/die based on neighbor counts (2-4 neighbors to survive or be born). Cell age is tracked and rendered as a color gradient (red → yellow).

## Commands

```bash
cargo build                          # Build
cargo run --bin gol3d                # Run (defaults: 25³ board, 500ms interval, 1000x800 window)
cargo run --bin gol3d -- -s 10 -i 300 -w 1200 -h 900  # Custom size/interval/window
cargo test --lib                     # Run unit tests (6 tests in lib.rs)
cargo bench                          # Run benchmarks (bencher crate, 30³ board)
cargo fmt --check                    # Check formatting
cargo clippy -- -D warnings          # Lint
RUST_LOG=debug cargo test            # Tests with debug logging
```

## Architecture

**Library (`src/lib.rs`)** — Pure game logic, no rendering:
- `Game` struct holds a 4D `ndarray::Array<usize, Ix4>` — dimensions are `[x, y, z, time_state]`
- **Double-buffering**: the 4th dimension has size 2; `state` field tracks which index (0 or 1) is current, the other stores the next generation. `swap_state()` flips between them.
- `Life` trait is the public API: `with_dimension(size)` (min 3), `init()` (seeds 9 cells in corner), `next()` (advances one generation)
- Cell values are `usize` ages (0 = dead, 1+ = alive with age), not booleans
- `pos!()` macro creates `Position` structs from tuples
- Neighbor calculation: `get_range()` handles boundaries, `neighbours_of()` generates up to 26 neighbors per cell

**Binary (`src/bin/gol3d.rs`)** — Rendering and CLI:
- Spawns a game thread that computes generations at the configured interval, sends `Game` clones over `mpsc::channel`
- Main thread renders with Kiss3D at display refresh rate, consuming game states via `try_recv()`
- `LivingCells` tracks active scene nodes; `render()` adds/removes/recolors cubes each frame
- CLI args via clap 2 macro (`clap_app!`)

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `ndarray` (+ rayon feature) | 4D array for game board |
| `kiss3d` | 3D rendering, window, camera |
| `clap` 2 | CLI argument parsing |
| `bencher` | Benchmark harness |

## Notes

- No Rust edition specified in Cargo.toml (defaults to 2015)
- CI is GitLab CI (`.gitlab-ci.yml`): build → test stages
- Benchmarks use the `bencher` crate with `harness = false` in Cargo.toml
