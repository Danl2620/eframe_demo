<!-- Copilot instructions for eframe_demo -->
# Quick orientation for AI coding agents

This repository is a minimal demo for `eframe` (an `egui` app) that can be built
both natively and for the web (WASM via `trunk`). Below are the most important
details an agent needs to be immediately productive.

**Big picture**
- **Native (desktop)**: code entry is `src/main.rs` (non-wasm `main` function) and
  the app implementation is in `src/app.rs` (`DemoApp`). See `src/lib.rs` which
  re-exports `DemoApp`.
- **Web (WASM)**: `#[cfg(target_arch = "wasm32")]` branch in `src/main.rs` uses
  `eframe::WebRunner` and expects a canvas with id `the_canvas_id` (must match
  `index.html`). `Trunk` is used to build/serve the web output.

**Key files to reference**
- `Cargo.toml` — dependencies, features (note `eframe` features like `persistence`,
  `glow`, etc.), release profile optimizations for wasm.
- `src/main.rs` — native vs wasm entry points and runtime logging (`env_logger` /
  `WebLogger`).
- `src/app.rs` — GUI layout, persistence usage (`eframe::APP_KEY`), and usage of
  `egui_code_editor` (example of project patterns for widgets/completer).
- `index.html` — trunk asset hooks, service-worker registration, and the canvas
  id `the_canvas_id` (hard-coded in `main.rs`).
- `assets/sw.js` — service-worker caching used in production; append `#dev` to
  URL to disable caching during development (see README).
- `flake.nix` / `Trunk.toml` — developer shell and trunk build config (useful for
  reproducing the exact dev environment).

**Build / run workflows (explicit examples)**
- Native debug: `cargo run` (or `cargo run --release` for release build).
- Web (dev):
  - `rustup target add wasm32-unknown-unknown`
  - `cargo install --locked trunk`
  - `trunk serve` -> open `http://127.0.0.1:8080/index.html#dev` (the `#dev` disables SW caching)
- Web (release deploy): `trunk build --release` -> uploads the generated `dist` directory.
- Nix dev-shell (reproducible environment): use the `flake.nix` devShell; it
  already installs `trunk` and the wasm target in nix (see `buildInputs`).

**Project-specific conventions / gotchas**
- The canvas id in `index.html` is hard-coded and must match `main.rs`:
  `the_canvas_id`.
- Service-worker `assets/sw.js` will cache the site; use `#dev` during local
  development to bypass cached assets. The loading spinner `#loading_text` is
  removed by wasm `main` when startup succeeds.
- Persistence: `DemoApp` derives `Serialize/Deserialize` (see `src/app.rs`)
  and uses `eframe::APP_KEY` to store state when `persistence` feature is enabled
  in `eframe` (configured in `Cargo.toml`). Do not change the key without
  adjusting load/save logic.
- Crate rename instructions (useful when templating): README documents all
  places to edit (`Cargo.toml`, `main.rs`, `index.html`, `assets/sw.js`).

**How to make small changes safely**
- UI tweaks belong in `src/app.rs`. Keep changes minimal; the example app is
  intentionally small and used as a template.
- When updating `egui`/`eframe`, update one version at a time and consult the
  upstream changelogs (the repo follows upstream `egui` closely).

If anything above is unclear or you want more detail (CI workflow, test
strategy, or help extracting more code examples), tell me which area to expand
and I'll iterate on the file.
