//! Homepage view
//! This should have a self-hiding section at the top for download tasks,
//! as well as some

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Layout, Widget};
use crate::terminal::view::View;

pub struct LandingPage {
    
}

impl LandingPage {
    pub fn new() -> Self {
        LandingPage {}
    }
}

impl View for LandingPage {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        
    }
}
