use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use ratatui::Frame;
use ratatui::prelude::Rect;
use tokio::sync::RwLock;

pub struct ProgressBar {
    started: Instant,
    pos: AtomicU64,
    len: u64,
    is_bytes: bool,
    msg: &'static str,
    display_rate: bool,
    displayed_rate: f64,
}

impl ProgressBar {
    pub fn new(len: u64, message: &'static str) -> Self {
        ProgressBar {
            started: Instant::now(),
            pos: AtomicU64::new(0),
            len,
            is_bytes: false,
            msg: message,
            display_rate: false,
            displayed_rate: 0.0,
        }
    }

    pub fn display_rate(mut self) -> Self {
        self.display_rate = true;
        self
    }

    pub fn shows_bytes(mut self) -> Self {
        self.is_bytes = true;
        self
    }

    pub fn inc(&self, delta: u64) {
        self.pos.fetch_add(delta, Ordering::AcqRel);
    }
}


pub struct ProgressBarManager {
    next_bar_id: AtomicU64,
    bars: RwLock<HashMap<u64, Arc<ProgressBar>>>,
}

impl ProgressBarManager {
    pub fn new() -> Self {
        ProgressBarManager {
            next_bar_id: AtomicU64::new(1),
            bars: RwLock::new(HashMap::new()),
        }
    }

    /// Reserve a bar id for your progress bar.
    pub fn take_next_id(&self) -> u64 {
        self.next_bar_id.fetch_add(1, Ordering::Relaxed)
    }




    /// Draw the progress bars section of the frame.
    pub fn render(&self, frame: &mut Frame, rect: Rect) {

    }
}
