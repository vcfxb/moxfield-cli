pub mod scryfall_pull;

pub trait ProgressBar {
    fn message(&self) -> String;

    fn pos(&self) -> u64;

    fn len(&self) -> u64;

    fn is_bytes(&self) -> bool;

    fn rate(&self) -> f64;
}
