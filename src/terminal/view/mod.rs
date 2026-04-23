//! Rendering for different views of the app.
//!
//! There are a few core views:
//! - The homepage, which should let a user update their config and add and remove moxfield
//!     entitites, as well as initiating a new scryfall refresh.
//! - The scryfall search page, which should serve as a fully offline capable scryfall search if
//!     possible
//! - The moxfield deck view page, which potentially has subpages for BOM, color identity, etc.
//! - You should not need a moxfield api key to use this tool -- it should be possible to paste in
//!     lists for decks and stuff.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;

pub mod home;

/// A view is like a widget, but it can also store state and can modify the app it's in.
///
/// It's expected that a view will render over the whole frame.
pub trait View
{
    fn render(&mut self, area: Rect, buf: &mut Buffer);
}

impl Widget for &mut dyn View {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        View::render(self, area, buf)
    }
}
