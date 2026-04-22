use std::io::Stdout;
use crossterm::cursor;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use tokio::task::JoinHandle;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crate::terminal::event_loop::{Event, EventLoop};
use crate::terminal::widget::fps::FpsState;

pub struct App {
    term: Terminal<CrosstermBackend<Stdout>>,
    event_loop: EventLoop,
    fps_state: FpsState,
}

impl App {
    pub fn new(frame_rate: f64, tick_rate: f64) -> color_eyre::Result<Self> {
        Ok(Self {
            event_loop: EventLoop::new(frame_rate, tick_rate),
            term: Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
            fps_state: FpsState::new(),
        })
    }

    async fn quit(self) -> color_eyre::Result<()> {
        crossterm::execute!(
            std::io::stdout(),
            LeaveAlternateScreen,
            cursor::Show,
            DisableMouseCapture
        )?;

        crossterm::terminal::disable_raw_mode()?;
        self.event_loop.stop().await?;

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
                    }

                    Event::Render => {
                        self.term.set_cursor_position((0, 0))?;
                        self.term.clear()?;
                        self.term.draw(|f| {
                            let block = Block::default().borders(Borders::ALL);
                            f.render_widget(block, f.area());
                            f.render_widget(&mut self.fps_state, f.area());

                            f.buffer_mut().set_string(
                                1,
                                0,
                                "oshibana",
                                Style::new().italic()
                            );
                        })?;
                    }

                    Event::Keyboard(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        kind: KeyEventKind::Press,
                        ..
                    }) => {
                        self.quit().await?;
                        break;
                    }

                    _other => {

                    },
                }
            }

            Ok(())
        })
    }
}
