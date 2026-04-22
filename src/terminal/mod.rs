
pub mod app;
pub mod event_loop;
pub mod view;
pub mod widget;

use std::io;

/// Blocking function that waits for the user to press any key and then returns.
pub fn wait_for_key_press() -> io::Result<()> {
    loop {
        if crossterm::event::read()?.is_key_press() {
            return Ok(());
        }
    }
}
