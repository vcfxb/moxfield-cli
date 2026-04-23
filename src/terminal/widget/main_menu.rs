//! Options available from the main menu landing page.

use strum::{EnumIter, IntoStaticStr};

#[derive(EnumIter, IntoStaticStr)]
pub enum MainMenuOption {
    ScryfallInfo,
    RefreshScryfall,
}


pub struct MainMenu {
    
}

