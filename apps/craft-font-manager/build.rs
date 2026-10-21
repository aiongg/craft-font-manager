//! Embeds craft-fonts' interface faces from the checkout `CRAFT_FONTS_DIR` names, so interface
//! text in Arabic, Hebrew, Thai and the other shipped scripts draws without system fonts; an empty
//! table when it is unset. CJK faces are embedded only as interface faces cut to the translation
//! catalogs, so none are embedded while the interface is English only.

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    if let Err(e) = craft_fonts_build::Embed::new().for_ui().write("craft_fonts.rs") {
        println!("cargo::error={e}");
    }
}
