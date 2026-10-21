# AGENTS.md: guide for AI agents and contributors

Craft Font Manager is the **Craft App Suite**'s font app. It lists the fonts the suite's apps see
(the system's and the suite's font store), and installs and removes Google Fonts families in the
font store. It is written in **Rust only**: a native egui/eframe desktop app. **No Tauri, Electron
or webview shells**, ever.

The product name is **Craft Font Manager** in user-facing text. Machine names stay lowercase: the
binary `craft-font-manager` and the app id `ai.storyteller.craft-font-manager`.

Standards shared across the Craft apps live in `../craftrules`; shared code lives in
[craft-libs](https://github.com/storytold/craft-libs).

## 1. What lives where

The interface and everything behind it is craft-libs' `craft-font-manager` crate: search and
filters over the Google Fonts index, installed fonts by family, the `Installer` interface with
craft-font-store's store, and (`egui` feature) `FontsPanel`, the whole interface as one panel.
ArtCraft Launcher shows the same panel as its Fonts tab. Changes to the interface, its strings,
the consent flow or its tests go to craft-libs (`crates/font-manager`, with its interface flows in
`crates/font-manager/tests/flows.rs`).

This repository is the standalone app around that panel:

```text
apps/
  craft-font-manager   package craft-font-manager-app, binary craft-font-manager: the eframe
                       window, the settings file, the real font store (or NoStore without a store
                       folder), and the interface fonts
```

The shell installs the interface fonts and direction with `install_interface_fonts`, passing
craft-fonts' faces that `build.rs` embeds when `CRAFT_FONTS_DIR` names a checkout, and enables
craft-ui's `system_fonts` so the system's fonts draw every other character (language names in
their own scripts). It saves the panel's `Settings` with eframe's storage.

## 2. Golden rules

- **Non-test code never panics.** No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!` or
  `unimplemented!`. Every crate root carries
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
- **No `unsafe`.** The workspace sets `unsafe_code = "forbid"`.
- **Nothing is requested at startup**, and downloads need the user's consent. The panel and
  craft-font-store enforce this; the shell adds no network use of its own.
- **One craft-libs revision.** Every craft-libs dependency in `Cargo.toml` names the same `rev`.
- **Assets need attribution.** Every bundled asset gets a row in `ATTRIBUTION.md` in the same
  change.
- Comments say only what the code can't: why, constraints, invariants.

## 3. Before you finish a task

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The suite's egui and winit forks are patched in `[patch.crates-io]` in `Cargo.toml`, pinned by
revision, the same revisions craft-libs is tested with.
