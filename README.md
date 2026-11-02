# RustCraft v0.2 (experimental)

Independent Minecraft-inspired voxel sandbox written in Rust. This extends the v0.1 macroquad foundation. `src/legacy_v01.rs` is retained as a reference copy of the original gameplay implementation.

## Features
- In-game launcher/home screen, settings, world browser and pause menu.
- Create named worlds (`World_1`, `World_2`, ...), select them, autosave edits every 30 seconds, and save on return to menu.
- Procedural terrain and trees with seeded deterministic generation.
- 16x16 chunk geometry: only exposed faces are submitted, one indexed mesh per chunk rather than one draw call per block. Builds one missing chunk mesh per frame; rebuilds neighboring chunk meshes on edits.
- Generated pixel-pattern texture atlas, nearest-neighbor sampling; no copyrighted Minecraft textures included.
- FPS/chunk diagnostics, movement, collision, mining, block placement and creative flight.

## Controls
Click gameplay to capture mouse; WASD movement; mouse look; Space jump; Shift sprint; 1-6 block selection; mouse wheel select; left click mine; right click place; F flight; Escape pause.

## Build on GitHub (no local Rust installation)
1. Create a GitHub repository.
2. Upload **the contents** of this directory, including hidden `.github/workflows/windows.yml`, `Cargo.toml`, and `src/main.rs`.
3. Open Actions > RustCraft Windows > Run workflow (or push a commit).
4. If build succeeds, download `RustCraft-v0.2-Windows` from the run's Artifacts section and extract `rustcraft.exe`.

## Build locally
Install Rust and the MSVC C++ build tools on Windows, then run `cargo run --release`.

## World files
World edits are saved as JSON in `%APPDATA%\\RustCraft\\worlds` on Windows. This is a prototype format, **not** Minecraft Anvil/NBT world compatibility. Back up saves before updating.

## Status / known limitations
This source package has **not been compiled or runtime-tested** in the authoring environment because Rust/Cargo are not installed. The GitHub workflow is the intended build/verification route. Please share any compilation error log for a patch. Chunk meshes are basic exposed-face meshing (not greedy meshing); texture atlas is generated procedurally. No standalone external launcher, inventory, mobs, multiplayer, or official Java protocol/world compatibility yet. World generation is intentionally original, not a reproduction of the latest Minecraft release. Minecraft is a trademark of Mojang/Microsoft; this project is independent.
