#[derive(rust_embed::RustEmbed)]
#[folder = "migrations/"]
struct Migrations;

fn main() {
    Migrations::iter().for_each(|file| {
        let path = file.as_ref();
        println!("cargo:rerun-if-changed={}", path);
    });
}
