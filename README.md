# Bevy Age of War

A small **Age of War**-inspired game prototype written in Rust with [Bevy](https://bevyengine.org/).

This repository is primarily a learning project: I wanted to experiment with an entity-component-system (ECS) architecture in Rust and use it to build a simple real-time strategy game.

## Current features

- Start screen and end-of-game screen
- Bevy ECS-based game systems and state management
- Multiple ages with different bases, units, and turrets
- Unit training queue and progress bar
- Unit movement, collisions, combat, health bars, and base health
- Camera scrolling during gameplay
- Background music

The project is still in progress. Turret placement and firing, along with other improvements, are tracked in [`TODO.md`](TODO.md).

## Requirements

- Rust and Cargo ([rustup.rs](https://rustup.rs/))
- A graphics environment supported by Bevy

## Running

Clone the repository and run:

```bash
cargo run
```

For an optimized build:

```bash
cargo run --release
```

## Controls

- Click **Play** to start
- Use the **Left Arrow** and **Right Arrow** keys to move the camera
- Use the HUD buttons to train units and interact with the game

## Project structure

- `src/age_of_war.rs` - application plugin group and shared game types
- `src/game/` - world, bases, units, combat, and health systems
- `src/hud/` - menus, unit queues, progress bars, and HUD interactions
- `src/start_screen/` - start screen and game-state transition
- `src/end_of_game/` - end screen and return-to-menu handling
- `assets/` - sprites, fonts, music, and other game assets

## Credits

The sprite assets are from [TruongHoangTungDuong/AgeOfWar](https://github.com/TruongHoangTungDuong/AgeOfWar/tree/main/Assets/Sprite/Sprites%20%C4%90A1).

