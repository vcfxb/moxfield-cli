use std::sync::LazyLock;
use directories::ProjectDirs;
use moxfield_cli::scryfall::client::ScryfallClient;
use moxfield_cli::terminal::app::App;

static SCRYFALL: LazyLock<ScryfallClient> = LazyLock::new(|| ScryfallClient::new());
static DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org", "vcfxb", env!("CARGO_PKG_NAME")).unwrap()
});

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // install global color_eyre panic handler for prettier panic handling
    color_eyre::install()?;

    // wrap with a function to wait for a keypress, so that terminals
    // spawned ephemerally don't immediately disappear on exit
    let original_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        original_panic_hook(panic_info);
        println!("[press any key to exit]");
        moxfield_cli::terminal::wait_for_key_press().unwrap();
    }));
    
    // now that we have a panic hook set, we can make an event loop and pass it to the app.
    let app = App::new(60.0, 6.0)?;
    app.run().await??;
    Ok(())
}
