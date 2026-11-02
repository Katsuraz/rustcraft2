# RustCraft Launcher v0.1

Independent Rust GUI launcher foundation for **legitimately owned Minecraft: Java Edition**. Not affiliated with Mojang or Microsoft.

## Included
- Native egui Windows desktop UI with Home, Installations, Settings, About tabs.
- Detect local Minecraft version metadata in `%APPDATA%\\.minecraft\\versions` (or a custom directory).
- Store game directory, Java path and RAM preferences in `%APPDATA%\\RustCraftLauncher\\settings.json`.
- Open the official Minecraft Launcher (via installed executable or Windows Minecraft URI).
- GitHub Actions Windows build and downloadable executable artifact.

## Limitations
- Does NOT yet launch Minecraft Java directly. The Play button opens the **official launcher**, where you sign in and press Play.
- Local version selection, Java path and RAM settings are not applied to the official launcher.
- No Microsoft OAuth, version downloads, modloader installation, or bundled game files yet.
- The executable has not been compiled in this environment; use GitHub Actions to validate and report errors.

## GitHub build without installing Rust
1. Create a GitHub repository.
2. Upload the **contents** of this folder to the repository root, including `.github/workflows/build.yml`.
3. Open Actions > Build RustCraft Launcher for Windows and wait for a green run.
4. Download `RustCraft-Launcher-Windows` under Artifacts, unzip, run the exe.

## Local build (optional)
`cargo run --release`

## Roadmap
1. Signed-in Microsoft/Minecraft account handling via supported OAuth flow.
2. Official version metadata, libraries, assets, Java runtime management and verification.
3. Direct launch with authenticated session and correct version arguments.
4. Fabric/Forge/NeoForge/Quilt installation and profile management.
5. Profiles, performance settings, update checks and crash logs.
