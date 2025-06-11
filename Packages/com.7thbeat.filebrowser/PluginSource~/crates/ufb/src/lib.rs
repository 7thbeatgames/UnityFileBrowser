mod filedialog;
mod opener;
mod utils;

pub use filedialog::*;
pub use opener::*;

mod entrypoint;

pub use entrypoint::*;

#[test]
fn test_file_dialog() {
    FileDialog::new().pick_file();
}

#[test]
fn test_file_opener() {
    reveal("Cargo.toml");
}
