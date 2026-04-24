use crate::terminal::app::view::View;
use crate::terminal::app::App;
use anymap3::Map;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use ratatui::prelude::{Constraint, Layout, Line, Margin};
use ratatui::style::Style;
use ratatui::widgets::Block;
use std::any::Any;
use tui_widget_list::hit_test::Hit;
use tui_widget_list::{ListBuilder, ListState, ListView};

pub struct Home;

const HIGHLIGHT_STYLE: Style = Style::new().white().on_blue();

#[derive(Debug)]
struct ListItem {
    text: &'static str,
    base_style: Style,
    highlight_style: Style,
    enabled: bool,
    action: fn(&mut App) -> color_eyre::Result<()>,
}

static MAIN_MENU: &'static [ListItem] = &[
    ListItem {
        text: "Scryfall Cache Info",
        highlight_style: HIGHLIGHT_STYLE,
        base_style: Style::new(),
        enabled: false,
        action: |app: &mut App| {
            todo!()
        }
    },

    ListItem {
        text: "Pull Fresh Scryfall Data",
        highlight_style: HIGHLIGHT_STYLE,
        base_style: Style::new(),
        enabled: false,
        action: |app: &mut App| {
            todo!()
        }
    },

    ListItem {
        text: "Version Info",
        highlight_style: HIGHLIGHT_STYLE,
        base_style: Style::new(),
        enabled: false,
        action: |app: &mut App| {
            todo!()
        }
    },

    ListItem {
        text: "Quit",
        highlight_style: Style::new().red().on_white(),
        base_style: Style::new(),
        enabled: true,
        action: |app: &mut App| {
            app.event_loop.send_quit()
        }
    }
];

#[derive(Default)]
pub struct HomeState {
    list_state: ListState,
}

fn get_home_state(view_states: &mut Map<dyn Any + Send + Sync>) -> &mut HomeState {
    view_states.entry().or_default()
}

impl View for Home {
    fn render(&self, app: &mut App) -> color_eyre::Result<()> {
        app.term.draw(|f| {
            super::render_global_block(f, &mut app.fps_state);

            let area = f.area().inner(Margin::new(1, 1));

            let layout = Layout::horizontal([
                Constraint::Fill(1), Constraint::Min(32), Constraint::Fill(1)
            ]);

            let centered_area = layout.split(area)[1];

            let layout = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Fill(3),
                Constraint::Fill(1),
            ]);

            let menu_area = layout.split(centered_area)[1];

            let builder = ListBuilder::new(|context| {
                let list_item = &MAIN_MENU[context.index];

                let selection_indicator = match context.is_selected {
                    true => "> ",
                    false => "  ",
                };

                let string = format!(
                    "{selection_indicator} {}: {}{}",
                    context.index + 1,
                    list_item.text,
                    if !list_item.enabled { " (disabled)" } else { "" }
                );

                let style = match (list_item.enabled, context.is_selected) {
                    (true, true) => list_item.highlight_style,
                    (true, false) => list_item.base_style,
                    (false, _) => Style::new().gray()
                };

                let line = Line::from(string).style(style);

                (line, 1)
            });

            let list_view = ListView::new(builder, MAIN_MENU.len())
                .block(Block::bordered().title_top(Line::from("Main Menu").centered()));

            f.render_stateful_widget(
                list_view,
                menu_area,
                &mut get_home_state(&mut app.view_states).list_state
            );
        })?;

        Ok(())
    }

    fn handle_key(&self, app: &mut App, ev: KeyEvent) -> color_eyre::Result<()> {
        match ev {
            KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                ..
            } => {
                get_home_state(&mut app.view_states).list_state.previous();
            },

            KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                ..
            } => {
                get_home_state(&mut app.view_states).list_state.next();
            }

            KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            } => 'enter: {
                let Some(index) = get_home_state(&mut app.view_states).list_state.selected else {
                    break 'enter;
                };

                if !MAIN_MENU[index].enabled {
                    break 'enter;
                }

                return (MAIN_MENU[index].action)(app);
            }

            _ => {}
        }

        Ok(())
    }

    fn handle_tick(&self, _: &mut App) -> color_eyre::Result<()> {
        Ok(())
    }

    fn handle_mouse(&self, app: &mut App, ev: MouseEvent) -> color_eyre::Result<()> {
        if ev.kind.is_moved() || ev.kind.is_down() {
            let list_state = &mut get_home_state(&mut app.view_states).list_state;
            let hit = list_state.hit_test(ev.column, ev.row);

            if let Some(Hit::Item(index)) = hit {
                match ev.kind {
                    MouseEventKind::Down(MouseButton::Left) => 'call: {
                        if !MAIN_MENU[index].enabled {
                            break 'call;
                        }

                        (MAIN_MENU[index].action)(app)?;
                    },

                    MouseEventKind::Moved => list_state.selected = Some(index),

                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn handle_paste(&self, _: &mut App, _: String) -> color_eyre::Result<()> {
        Ok(())
    }
}
