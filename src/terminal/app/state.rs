use std::sync::LazyLock;
use directories::ProjectDirs;

static DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org", "vcfxb", env!("CARGO_PKG_NAME")).unwrap()
});
