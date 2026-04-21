use std::io::Stdout;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use tokio::task::JoinHandle;
use crossterm::event::Event as CtEvent;
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers, KeyEventKind};
use crate::terminal::event_loop::EventLoop;

pub struct App {
    term: Terminal<CrosstermBackend<Stdout>>,
    event_loop: EventLoop,
}

/*
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, cursor::Show)?;
 */

impl App {
    pub fn new(frame_rate: f64) -> color_eyre::Result<Self> {
        Ok(Self {
            event_loop: EventLoop::new(frame_rate),
            term: Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
        })
    }

    pub fn run(mut self) -> JoinHandle<color_eyre::Result<()>> {
        tokio::spawn(async move {
            crossterm::terminal::enable_raw_mode()?;
            crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide)?;

            while let Some(ev) = self.tui.next().await {
                match ev {
                    Event::Render => {
                        self.tui.term.set_cursor_position((0, 0)).unwrap();
                        self.tui.term.clear().unwrap();
                        self.tui.term.draw(|f: &mut Frame| {
                            let block = Block::default().borders(Borders::ALL);
                            f.render_widget(block, f.area());
                        }).unwrap();
                    }

                    Event::UserInput(CtEvent::Key(key_ev)) => match key_ev {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            kind: KeyEventKind::Press,
                            ..
                        } => {
                            self.tui.exit().await?;
                            break;
                        },

                        _ => {}
                    }

                    other => { dbg!(other); },
                }
            }

            Ok(())
        })
    }
}
