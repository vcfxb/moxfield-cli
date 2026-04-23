use crate::terminal::event_loop::{Event, EventLoop};
use crate::terminal::widget::fps::FpsState;
use crossterm::cursor;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::*;
use ratatui::widgets::Block;
use std::io::Stdout;
use tokio::task::JoinHandle;
use crate::terminal::widget::main_menu::MainMenu;

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
        self.event_loop.stop().await?;

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
                    }

                    Event::Render => {
                        self.term.set_cursor_position((0, 0))?;
                        self.term.draw(|f| {
                            // full block
                            let block = Block::bordered()
                                .title_top(Line::from("oshibana").italic());

                            // render full block + fps
                            f.render_widget(&block, f.area());
                            f.render_widget(&mut self.fps_state, f.area());

                            // create layout for rendering

                            // if there are progress bars, render a block for those

                            // // render active view
                            // let view: &mut dyn View = self.view.as_mut();
                            // f.render_widget(view, f.area());
                            // render main menu
                            f.render_widget(&MainMenu {}, block.inner(f.area()));

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

                    _other => {}
                }
            }

            Ok(())
        })
    }
}
