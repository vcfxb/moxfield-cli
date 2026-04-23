//! Scryfall cache state tracking.

use std::path::PathBuf;
use std::sync::LazyLock;
use crate::terminal::state::DIRECTORIES;

static SCRYFALL_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    DIRECTORIES.data_dir().join("scryfall_cache")
});



pub struct ScryfallCacheService {

}


