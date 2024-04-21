
_default:
    just --list

# Run with dynamic linking for faster compiles
run:
    cargo run --features bevy/dynamic_linking

