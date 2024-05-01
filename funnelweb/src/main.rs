use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use funnelweb_config::Config as FunnelWebConfig;
use funnelweb_config::ImapConfig;

use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::Style,
    symbols,
    widgets::{Block, Borders, Paragraph, Tabs},
};
use std::io::{stdout, Result};
use std::thread;

use thingbuf::StaticThingBuf;

use funnelweb_imap::ImapItem;
use funnelweb_imap_widget::{
    ImapConnection, ImapConnectionState, ImapConnections, ImapConnectionsWidget,
    SelectedImapConnection,
};

#[derive(Debug)]
enum IoItem {
    Imap(ImapItem),
}

static IO_QUEUE_OUT: StaticThingBuf<IoItem, 1048> = StaticThingBuf::new();
static IO_QUEUE_IN: StaticThingBuf<IoItem, 1048> = StaticThingBuf::new();

fn io_thread() {}

fn main() {
    let config = FunnelWebConfig::from_file("funnel.toml").unwrap();

    thread::scope(|s| {
        s.spawn(|| {});
    });

    ui_main(config).unwrap();
}

fn ui_main(config: FunnelWebConfig) -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut switch_imap_tab = false;

    let mut select_connection = true;

    let mut imap_connections = ImapConnections::new();

    config.imap.iter().for_each(|imap_config| {
        let new_connection = ImapConnection::new(imap_config.name.clone());
        imap_connections.push(new_connection);
    });

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            if switch_imap_tab {
                let imap_tabs: Vec<String> =
                    config.imap.iter().map(|imap| imap.name.clone()).collect();

                let tabs = Tabs::new(imap_tabs)
                    .block(Block::default().title("funnelWeb").borders(Borders::ALL))
                    .style(Style::default().white())
                    .highlight_style(Style::default().yellow())
                    .select(2)
                    .divider(symbols::DOT)
                    .padding("->", "<-");

                frame.render_widget(tabs, area);
            }

            if select_connection {
                let widget = ImapConnectionsWidget;
                frame.render_stateful_widget_ref(widget, frame.size(), &mut imap_connections);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            let event_in = event::read()?;
            if select_connection {
                if imap_connections.handle_event(event_in) {
                    select_connection = false;
                }
            }
            if let event::Event::Key(key) = event_in {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
