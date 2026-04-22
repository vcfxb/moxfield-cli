//! Core event loop -- fires events for rendering, user input. Picked up by app render loop.
//!
//! We need a little wrapper thingy to get ratatui to sync up with the tokio runtime.

use crossterm::event::{Event as CtEvent, KeyEvent, MouseEvent};
use futures::prelude::*;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_util::future::FutureExt;
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub enum Event {
    /// Sent to start each draw.
    Render,

    /// Sent every app tick
    Tick,

    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    Resize(u16, u16),
}

pub struct EventLoop {
    /// Event loop task
    ev_loop: OnceLock<JoinHandle<()>>,

    /// Token used to exit the event loop gracefully.
    cancellation_token: CancellationToken,

    // event queue
    event_tx: UnboundedSender<Event>,
    event_rx: UnboundedReceiver<Event>,

    pub frame_rate: f64,
    pub tick_rate: f64,
}

impl EventLoop {
    /// Create a new event loop. It will not be started until you start it.
    pub fn new(frame_rate: f64, tick_rate: f64) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            ev_loop: OnceLock::new(),
            cancellation_token: CancellationToken::new(),
            event_tx,
            event_rx,
            frame_rate,
            tick_rate,
        }
    }

    /// Start this event loop.
    pub fn start(&mut self) -> color_eyre::Result<()> {
        // get an event stream from cross term.
        let mut event_stream = crossterm::event::EventStream::new();

        let cancellation_token = self.cancellation_token.clone();
        let event_tx = self.event_tx.clone();

        let frame_duration = Duration::from_secs_f64(1.0 / self.frame_rate);
        let mut frame_interval = tokio::time::interval(frame_duration);

        let tick_duration = Duration::from_secs_f64(1.0 / self.tick_rate);
        let mut tick_interval = tokio::time::interval(tick_duration);

        self.ev_loop.get_or_init(move || {
            tokio::spawn(async move {
                'ev_loop: loop {
                    tokio::select! {
                        // cancel
                        _ = cancellation_token.cancelled() => {
                            break 'ev_loop;
                        }

                        // tick
                        _ = tick_interval.tick() => {
                            event_tx.send(Event::Tick).unwrap();
                        }

                        // render event
                        _ = frame_interval.tick() => {
                            event_tx.send(Event::Render).unwrap();
                        }

                        // user input event
                        Some(event_result) = event_stream.next().fuse() => {
                            let ev = match event_result.expect("crossterm IO error") {
                                CtEvent::Key(k) => Event::Keyboard(k),
                                CtEvent::Mouse(m) => Event::Mouse(m),
                                CtEvent::Resize(cols, rows) => Event::Resize(cols, rows),
                                CtEvent::Paste(s) => Event::Paste(s),
                                // ignore all other events.
                                _ => continue 'ev_loop,
                            };

                            event_tx.send(ev).expect("sent event");
                        }
                    }
                }
            })
        });

        Ok(())
    }

    pub async fn stop(mut self) -> color_eyre::Result<()> {
        // if there is no event loop to cancel, we don't need to do anything.
        let Some(ev_loop_handle) = self.ev_loop.take() else {
            return Ok(());
        };

        self.cancellation_token.cancel();
        if ev_loop_handle
            .timeout(Duration::from_millis(100))
            .await
            .is_err()
        {
            panic!("Event loop did not stop in 100 millis")
        }

        Ok(())
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }
}
