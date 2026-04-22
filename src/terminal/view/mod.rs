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

pub mod home;