//! Fps widget -- this widget

use colorgrad::Gradient;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, Layout, Style, Widget};
use ratatui::style::Color;
use std::time::Instant;
use crate::utils::ring_buffer::RingBuffer;

pub struct FpsState {
    /// Ring buffer of instants representing times of the last few frames.
    ring_buffer: RingBuffer<Instant>,
    displayed_fps: f64,
}

impl FpsState {
    pub fn new() -> Self {
        FpsState {
            ring_buffer: RingBuffer::new(64, Instant::now()),
            displayed_fps: 0.0,
        }
    }

    pub fn app_tick(&mut self) {
        self.displayed_fps = self.average_fps();
    }

    pub fn render_tick(&mut self) {
        self.ring_buffer.push(Instant::now());
    }

    pub fn average_fps(&self) -> f64 {
        let Some(start) = self.ring_buffer.get(0) else {
            return 0.0;
        };

        let elapsed = start.elapsed().as_secs_f64();
        let count = self.ring_buffer.len();

        (count as f64) / elapsed
    }
}

impl<'a> Widget for &'a mut FpsState {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.render_tick();

        let v_rects = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).split(area);

        let h_rects = Layout::horizontal([Constraint::Percentage(100), Constraint::Min(10)])
            .split(v_rects[0]);

        let rect = h_rects[1];
        let string = format!("{:.2} fps", self.displayed_fps);

        let gradient = colorgrad::preset::rd_yl_gn();
        let (g_start, g_end) = gradient.domain();
        let color_index = ((self.displayed_fps as f32) / 60.0 * (g_end - g_start) + g_start)
            .clamp(g_start, g_end);
        let color = gradient.at(color_index);
        let as_24bit = color.to_rgba8();

        let style = Style::new().fg(Color::Rgb(as_24bit[0], as_24bit[1], as_24bit[2]));
        buf.set_stringn(rect.x, rect.y, string, 9, style);
    }
}
