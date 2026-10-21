# Craft Font Manager

The fonts of the Craft App Suite in one place: every font the suite's apps can use, and the Google
Fonts collection to add to them. A native Rust app (egui), MIT OR Apache-2.0.

## What it does

- **Installed**: every font the suite's apps see, grouped by family: the system's fonts and the
  fonts in the suite's font store. Each family shows where it comes from, its styles, and a sample
  line drawn in the face itself. Fonts in the font store can be removed; system fonts are removed
  with the system's own tools.
- **Catalogue**: the Google Fonts collection. Search by name, filter by category, language (by
  its English name or its own name) and script, and set the sample text and size. Installed
  families show their styles at once; for the others, **Preview** downloads the family for this
  session without installing it.
- **Install and remove** with progress. Installed fonts go to the suite's font store, a folder
  every Craft app reads and no other program does:

  | Platform | Folder |
  |---|---|
  | macOS | `~/Library/Application Support/Craft Fonts` |
  | Windows | `%LOCALAPPDATA%\Craft Fonts` |
  | Linux and the BSDs | `$XDG_DATA_HOME/craft-fonts` (`~/.local/share/craft-fonts`) |

## Network use

Nothing is downloaded until you allow it. The **Allow downloads** switch is off by default; the
first Install or Preview asks first. Files come from
[google/fonts](https://github.com/google/fonts) at a pinned commit, through jsDelivr (raw GitHub as
the fallback), and each one is checked against the index's size and sha256 before it is used.
The setting belongs to the suite's font store, so it holds for every Craft app. Browsing and
searching the catalogue work offline: the app bundles the
[Google Fonts index](https://github.com/aiongg/google-fonts-index).

## Building

```sh
cargo run -p craft-font-manager --release
```

Optional build inputs:

| Variable | Effect |
|---|---|
| `CRAFT_FONTS_DIR` | A [craft-fonts](https://github.com/storytold/craft-fonts) checkout: its interface faces are embedded, so non-Latin interface text draws without system fonts |

`craft-font-manager --version` prints `craft-font-manager <version>`.

## Embedding

The interface is one egui panel, `craft_font_manager_ui::FontsPanel`, with no window or eframe of
its own; ArtCraft Launcher shows it as a tab. The host builds it from a font catalog, a font store
and the index, and calls `ui` each frame:

```rust
let catalog = Arc::clone(Catalog::global());
catalog.scan_in_background();
let installer: Arc<dyn Installer> = Arc::new(SuiteStore::for_user(Arc::clone(&catalog)).unwrap());
let options = Options { catalog, installer, index: index::bundled().map_err(|e| e.to_string()), runner: Runner::Thread };
let mut panel = FontsPanel::new(options, Settings::default());
// each frame
panel.ui(ui);
```

`settings()` holds what the host saves (sample text and size). The download consent is the font
store's own setting, shared by every Craft app: `downloads_allowed()` and
`set_downloads_allowed(on)` read and set it for a host's settings page.

## Testing

```sh
cargo test --workspace
CRAFT_FONT_MANAGER_SNAPSHOTS=<dir> cargo test -p craft-font-manager-ui --test flows screenshot -- --test-threads=1
```

The interface tests drive the app with `egui_kittest` against a small index, a font folder made
per test and a fake font store.

## Licence

MIT OR Apache-2.0, at your option. Bundled material and its licences: [`NOTICE`](NOTICE) and
[`ATTRIBUTION.md`](ATTRIBUTION.md).
