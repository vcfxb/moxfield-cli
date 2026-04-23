use std::sync::LazyLock;
use directories::ProjectDirs;

pub mod scryfall_cache;
pub mod progress_bar_manager;

static DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org", "vcfxb", env!("CARGO_PKG_NAME")).unwrap()
});
