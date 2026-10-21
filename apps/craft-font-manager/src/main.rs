#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
// No console window behind the app in Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Craft Font Manager: the fonts installed for the Craft App Suite, and Google Fonts to add to
//! the suite's font store.

use std::sync::Arc;

use craft_font_manager::index;
use craft_font_manager::store::{Installer, NoStore, SuiteStore};
use craft_font_manager::{Catalog, FontsPanel, Options, Runner, Settings, install_interface_fonts};
use craft_ui::fonts::CraftFace as EmbeddedFace;

/// craft-fonts' interface faces (build.rs); empty unless built with `CRAFT_FONTS_DIR`.
static CRAFT_FONTS: &[EmbeddedFace] = include!(concat!(env!("OUT_DIR"), "/craft_fonts.rs"));

const SETTINGS_KEY: &str = "craft-font-manager.settings";

struct Shell {
    app: FontsPanel,
}

impl eframe::App for Shell {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::no_frame().show(ui, |ui| self.app.ui(ui));
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, SETTINGS_KEY, self.app.settings());
    }
}

fn main() -> eframe::Result {
    // The launcher reads the installed version from this line.
    if std::env::args().nth(1).as_deref() == Some("--version") {
        println!("craft-font-manager {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    env_logger::init();
    let catalog = Arc::clone(Catalog::global());
    catalog.scan_in_background();
    let index = index::bundled().map_err(|e| e.to_string());
    let installer: Arc<dyn Installer> = match SuiteStore::for_user(Arc::clone(&catalog)) {
        Some(store) => Arc::new(store),
        None => Arc::new(NoStore),
    };
    let viewport = egui::ViewportBuilder::default()
        .with_title("Craft Font Manager")
        .with_app_id("ai.storyteller.craft-font-manager")
        .with_inner_size([1180.0, 760.0])
        .with_min_inner_size([720.0, 480.0]);
    let options = eframe::NativeOptions { viewport, ..Default::default() };
    eframe::run_native(
        "Craft Font Manager",
        options,
        Box::new(move |cc| {
            install_interface_fonts(&cc.egui_ctx, CRAFT_FONTS);
            let settings: Settings = cc.storage.and_then(|s| eframe::get_value(s, SETTINGS_KEY)).unwrap_or_default();
            let options = Options { catalog, installer, index, runner: Runner::Thread };
            Ok(Box::new(Shell { app: FontsPanel::new(options, settings) }))
        }),
    )
}
