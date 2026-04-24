use crate::terminal::event_loop::{Event, EventLoop};
use anymap3::Map;
use crossterm::cursor;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::*;
use std::any::Any;
use std::io::Stdout;
use tokio::task::JoinHandle;
use view::View;
use widget::fps::FpsState;

pub mod view;
pub mod widget;
pub mod state;

pub struct App {
    term: Terminal<CrosstermBackend<Stdout>>,
    event_loop: EventLoop,
    fps_state: FpsState,
    view: &'static dyn View,
    view_states: Map<dyn Any + Send + Sync>,
}

impl App {
    pub fn new(frame_rate: f64, tick_rate: f64) -> color_eyre::Result<Self> {
        Ok(Self {
            event_loop: EventLoop::new(frame_rate, tick_rate),
            term: Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
            fps_state: FpsState::new(),
            view: &view::home::Home,
            view_states: Map::new(),
        })
    }

    fn quit(&mut self) -> color_eyre::Result<()> {
        crossterm::execute!(
            std::io::stdout(),
            LeaveAlternateScreen,
            cursor::Show,
            DisableMouseCapture
        )?;

        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    pub fn run(mut self) -> JoinHandle<color_eyre::Result<()>> {
        tokio::spawn(async move {
            crossterm::terminal::enable_raw_mode()?;
            crossterm::execute!(
                std::io::stdout(),
                EnterAlternateScreen,
                cursor::Hide,
                EnableMouseCapture
            )?;

            self.event_loop.start()?;

            while let Some(ev) = self.event_loop.next_event().await {
                match ev {
                    Event::Tick => {
                        self.fps_state.app_tick();
                        self.view.handle_tick(&mut self)?;
                    }

                    Event::Render => {
                        self.term.set_cursor_position((0, 0))?;
                        self.view.render(&mut self)?;
                    }

                    Event::Quit | Event::Keyboard(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        self.quit()?;
                        self.event_loop.stop().await?;
                        break;
                    }

                    Event::Keyboard(key_ev) => {
                        self.view.handle_key(&mut self, key_ev)?;
                    }

                    Event::Mouse(mouse_ev) => {
                        self.view.handle_mouse(&mut self, mouse_ev)?;
                    }

                    Event::Paste(s) => {
                        self.view.handle_paste(&mut self, s)?;
                    }

                    Event::Resize(_, _) => {},
                }
            }

            Ok(())
        })
    }
}
