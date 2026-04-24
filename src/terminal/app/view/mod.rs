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

use crate::terminal::app::widget::fps::FpsState;
use crate::terminal::app::App;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::prelude::{Line, Stylize};
use ratatui::widgets::Block;
use ratatui::Frame;

pub mod home;

/// Views are intended to be ZSTs that effectively operate as an FSM for handling renders
/// and events.
pub trait View: Send + Sync {
    fn render(&self, app: &mut App) -> color_eyre::Result<()>;

    fn handle_key(&self, app: &mut App, ev: KeyEvent) -> color_eyre::Result<()>;

    fn handle_tick(&self, app: &mut App) -> color_eyre::Result<()>;

    fn handle_mouse(&self, app: &mut App, ev: MouseEvent) -> color_eyre::Result<()>;

    fn handle_paste(&self, app: &mut App, s: String) -> color_eyre::Result<()>;
}

pub fn render_global_block(frame: &mut Frame, fps_state: &mut FpsState) {
    // full block
    let block = Block::bordered();

    // center title bar
    let mut title_bar = frame.area();
    title_bar.height = 1;
    title_bar.width = title_bar.width.saturating_sub(2);
    title_bar.x += 1;

    let title = Line::from("Oshibana Terminal Interface")
        .italic()
        .centered()
        .black()
        .on_white();

    frame.render_widget(&block, frame.area());
    frame.render_widget(&title, title_bar);
    frame.render_widget(fps_state, frame.area());
}
