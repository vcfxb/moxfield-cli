//! Options available from the main menu landing page.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Style, Widget};
use ratatui::widgets::{Block, List};
use strum::{Display, EnumIter, IntoEnumIterator, IntoStaticStr};

#[derive(EnumIter, Display)]
pub enum MainMenuOption {
    #[strum(to_string = "Scryfall Pull Info")]
    ScryfallInfo,

    #[strum(to_string = "Refresh Scryfall")]
    RefreshScryfall,

    #[strum(to_string = "Directories Info")]
    DirectoryInfo,
}


pub struct MainMenu {
    
}

impl Widget for &MainMenu {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let list = List::new(MainMenuOption::iter().map(|opt| opt.to_string()))
            .highlight_symbol(">>")
            .highlight_style(Style::new().black().on_white().bold());

        list.render(area, buf);
    }
}
