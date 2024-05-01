use crate::ImapConnection;
use crate::ImapConnectionState;

use crossterm::event::{Event, KeyEventKind, KeyCode};

use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::StatefulWidgetRef;

#[derive(Debug, Default)]
pub struct SelectedImapConnection {
    pub selected: Option<String>,
    pub changed: Vec<String>,
}

#[derive(Debug)]
pub struct ImapConnections {
    pub connections: Vec<ImapConnection>,
    pub state: SelectedImapConnection,
}

impl ImapConnections {
    pub fn new() -> Self {
        Self {
            connections: vec![],
            state: SelectedImapConnection::default(),
        }
    }
    pub fn push(&mut self, connection: ImapConnection) {
        self.connections.push(connection);
    }
    pub fn handle_event(&mut self, event: &Event) -> bool {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return false;
            }
            let selection: u32 = match key.code {
                KeyCode::Char(c) => {
                    match c.to_digit(10) {
                        Some(d) => d,
                        None => return false,
                    }
                },
                _ => return false,
            };

            if let Some(selection_exists) = self.connections.get(selection as usize) {
                self.state.selected = Some(selection_exists.name.clone());
                return true;
            }
        }
        false
    }
}

pub struct ImapConnectionsWidget;

impl StatefulWidgetRef for ImapConnectionsWidget {
    type State = ImapConnections;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut c = 0;
        let style = Style::new().fg(Color::LightGreen).bg(Color::Magenta);
        buf.set_string(1, 1, "Select connection", style);
        for conn in &state.connections {
            c += 1;
            let render_line = &conn.name;
            buf.set_string(c + 1, 3, render_line, style);
        }
    }
}
