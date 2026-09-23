use std::{
    io, panic,
    sync::{mpsc, Arc, Mutex},
    time::Duration,
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame, Terminal,
};

use crate::{app::App, common::Command, hack_computer::HackComputer};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stderr>>,
    exit: bool,
}

impl Tui {
    pub fn new() -> io::Result<Self> {
        let backend = CrosstermBackend::new(io::stderr());
        let terminal = Terminal::new(backend)?;
        Ok(Tui {
            terminal,
            exit: false,
        })
    }

    pub fn enter(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn run(
        &mut self,
        hack_computer: &Arc<Mutex<HackComputer>>,
        tx: &mpsc::Sender<Command>,
    ) -> io::Result<()> {
        while !self.exit {
            let guard = hack_computer.lock().unwrap();
            let mut app = App::new(&guard);

            self.terminal.draw(|frame| {
                render(&mut app, frame);
            })?;

            drop(guard);
            self.handle_events(tx)?;
        }
        Ok(())
    }

    fn handle_events(&mut self, tx: &mpsc::Sender<Command>) -> io::Result<()> {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event, tx);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent, tx: &mpsc::Sender<Command>) {
        match event.code {
            KeyCode::Char('q') => {
                let _ = tx.send(Command::Quit);
                self.exit = true;
            }
            KeyCode::Char('r') => {
                let _ = tx.send(Command::Run);
            }
            KeyCode::Char('s') => {
                let _ = tx.send(Command::Step);
            }
            _ => {}
        }
    }

    fn reset() -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> io::Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

fn render(app: &mut App, frame: &mut Frame) {
    let [header_area, memory_area, registers_area, instructions_area] = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(frame.area());

    render_header(frame, header_area);
    render_memory(app, frame, memory_area);
    render_registers(app, frame, registers_area);
    render_instructions(frame, instructions_area);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Line::from("hxcemu".bold());
    frame.render_widget(header.centered(), area);
}

fn render_registers(app: &mut App, frame: &mut Frame, area: Rect) {
    let [areg_area, dreg_area, pc_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .areas(area);

    frame.render_widget(
        Paragraph::new(format!("{:06x}", app.a_reg))
            .block(Block::default().title("A").borders(Borders::ALL)),
        areg_area,
    );
    frame.render_widget(
        Paragraph::new(format!("{:06x}", app.d_reg))
            .block(Block::default().title("D").borders(Borders::ALL)),
        dreg_area,
    );
    frame.render_widget(
        Paragraph::new(format!("{:06x}", app.pc))
            .block(Block::default().title("PC").borders(Borders::ALL)),
        pc_area,
    );
}

fn render_memory(app: &mut App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let visible_rows = area.height.saturating_sub(3);

    render_memory_table(
        frame,
        chunks[0],
        "RAM",
        "Hex",
        |addr| {
            let v = app.hack_computer.ram(addr);
            format!("{:04x}", v)
        },
        app.a_reg,
        visible_rows,
        Some(app.a_reg),
    );
    render_memory_table(
        frame,
        chunks[1],
        "ROM",
        "Bin",
        |addr| format!("{:016b}", app.hack_computer.rom(addr)),
        app.pc,
        visible_rows,
        Some(app.pc),
    );
}

fn render_memory_table<F: Fn(u16) -> String>(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    value_column_title: &str,
    format_value: F,
    center: u16,
    visible_rows: u16,
    highlight: Option<u16>,
) {
    let start = center.saturating_sub(visible_rows / 2);

    let rows: Vec<Row> = (0..visible_rows)
        .map(|i| {
            let addr = start + i;
            let style = if Some(addr) == highlight {
                Style::default().fg(Color::White).bg(Color::Black)
            } else {
                Style::default()
            };
            Row::new(vec![format!("{addr:06x}"), format_value(addr)]).style(style)
        })
        .collect();

    let table = Table::new(rows, [Constraint::Length(7), Constraint::Length(16)])
        .header(
            Row::new(vec!["Address", value_column_title]).style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(table, area);
}

fn render_instructions(frame: &mut Frame, area: Rect) {
    let instructions = Line::from("Run:r | Step:s | Quit:q");
    frame.render_widget(instructions.centered(), area);
}
