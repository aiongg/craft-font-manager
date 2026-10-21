# AGENTS.md: guide for AI agents and contributors

Craft Font Manager is the **Craft App Suite**'s font app. It lists the fonts the suite's apps see
(the system's and the suite's font store), and installs and removes Google Fonts families in the
font store. It is written in **Rust only**: a native egui/eframe desktop app. **No Tauri, Electron
or webview shells**, ever.

The product name is **Craft Font Manager** in user-facing text. Machine names stay lowercase:
crates and the binary (`craft-font-manager`, `craft-font-manager-*`) and the app id
`ai.storyteller.craft-font-manager`.

Standards shared across the Craft apps live in `../craftrules`; shared code lives in
[craft-libs](https://github.com/storytold/craft-libs).

## 1. Workspace map

```text
crates/
  core   craft-font-manager-core   no UI: search and filters over the Google Fonts index,
                                   installed fonts by family, the font store interface (Installer)
                                   with craft-font-store's store (SuiteStore) and a fake (FakeStore)
  ui     craft-font-manager-ui     the egui interface as one embeddable panel (FontsPanel),
                                   interface strings, the standalone app's interface fonts
apps/
  craft-font-manager               desktop shell (eframe): window, settings file, the real store
```

`core` has no egui. `ui` uses `core` and has no eframe: `FontsPanel` fills whatever `Ui` it is
given, so ArtCraft Launcher can show it as its Fonts tab and the shell as its window. It assumes no
window, menu, theme or interface fonts of its own: the host installs fonts and direction
(`install_interface_fonts` is the shell's), passes the catalog, store and index in `Options`,
saves `Settings`, and reads or sets the download consent with `downloads_allowed` and
`set_downloads_allowed`. The faces it draws samples in are its own egui font families, added and
taken out without touching the host's.

## 2. Where fonts come from

- **Installed fonts** come from `craft-font-catalog` (craft-libs): the platform's font folders and
  font service, plus the font store folder. The Installed tab lists System and Font store faces
  by family; other sources are left out.
- **The catalogue** is the Google Fonts index from `aiongg/google-fonts-index` at the commit
  craft-font-store pins, compiled in through its `bundled-index` feature. Browsing, searching and
  filtering never touch the network.
- **Installs, removals and preview downloads** go through the `Installer` trait
  (`crates/core/src/store.rs`). `SuiteStore` is craft-font-store's `FontStore`: verified installs
  into the store folder, removal, the download setting, and a catalog rescan after each change.
  Previews download with the store's HTTP client and `verify_font`, and stay in memory.
  `FakeStore` serves files from a table and writes them into a folder like the real store; the
  tests use it.

## 3. Golden rules

### Never crash

- **Non-test code never panics.** No `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!` or
  `unimplemented!`. Every crate root carries
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.
- **No `unsafe`.** The workspace sets `unsafe_code = "forbid"`.
- Index data, font files and anything from the network are hostile: `get()` instead of indexing,
  capped sizes, errors the user can act on.

### Network and consent

- Nothing is requested at startup.
- Downloads (installing and previewing) need the user's consent: the "Allow downloads" switch,
  off by default, or confirming the dialog an Install or Preview click opens. The setting is the
  font store's (`settings.json` in the store folder), so it holds for every Craft app. The app
  checks it before starting a download, and the store refuses downloads while it is off.
- Files come only from google/fonts at the index's pinned commit, through jsDelivr with raw
  GitHub as the fallback, and are verified (size, sha256, parses as a font) before use.
- Previews are kept in memory for the session and never installed.

### Languages

Every user-facing string goes through craft-i18n: `tl!("…")` for literals, `i18n::fmt` for
`{placeholders}`, `i18n::tn` for counts (`crates/ui/src/i18n.rs`). The English text is the lookup
key. The interface is English until the app's catalog rows exist in craft-i18n; product names
stay in English.

Interface fonts come from craft-ui's `UiFonts`: egui's default faces, craft-fonts' faces when
`CRAFT_FONTS_DIR` names a checkout at build time (`crates/ui/build.rs`), then the system's fonts
for every other character (language names in their own scripts). The interface direction follows
the language through `craft_ui::direction`; use its helpers (`direction::horizontal`,
`direction::end_panel`) rather than naming left and right.

### Other rules

1. **Assets need attribution.** Every bundled asset gets a row in `ATTRIBUTION.md` in the same
   change.
2. **Tests are the gate.** Each behaviour gets a test that would catch a regression: core logic as
   unit tests, interface flows as `egui_kittest` tests in `crates/ui/tests/`.
3. Comments say only what the code can't: why, constraints, invariants.

## 4. Before you finish a task

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The suite's egui fork is patched in `[patch.crates-io]` in `Cargo.toml`, pinned by revision,
with the winit revision egui's own `Cargo.lock` uses.
