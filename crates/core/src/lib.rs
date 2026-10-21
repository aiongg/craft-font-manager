#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

//! Craft Font Manager without a UI: the bundled Google Fonts index, searching it, the installed
//! fonts by family, and the font store interface the app installs and removes through.

pub mod filter;
pub mod index;
pub mod installed;
pub mod store;

pub use craft_font_catalog;
pub use craft_font_catalog::{Catalog, FaceInfo, FontEnv, Source};
