#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

//! Craft Font Manager's interface, in egui: the installed fonts, the Google Fonts catalogue, and
//! installing and removing through the font store.
//!
//! [`FontsPanel`] is the whole interface as one panel that fills the `Ui` it is given, with no
//! window, menu or eframe of its own, so ArtCraft Launcher can show it as a tab and the standalone
//! app as its window. The host passes in the catalog, the store and the index ([`Options`]),
//! saves [`Settings`], and reads or sets the download consent through the panel.

mod i18n;

mod app;
mod faces;
mod jobs;

pub use app::{FontsPanel, Options, Settings, Tab, install_interface_fonts};
pub use jobs::Runner;
