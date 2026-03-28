use sysinfo::{System, Disks};
use std::time::Duration;
use std::io;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Gauge},
    layout::{Layout, Constraint, Direction},
    style::{Color, Style},
    Terminal,
};
use crossterm::{
    execute,
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        let cpu = sys.global_cpu_info().cpu_usage();
        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();
        let mem_percent = (used_mem as f64 / total_mem as f64 * 100.0) as u16;

        // Disk usage
        let disks = Disks::new_with_refreshed_list();
        let (disk_used, disk_total) = disks.iter().fold((0u64, 0u64), |(used, total), d| {
            (used + d.total_space() - d.available_space(), total + d.total_space())
        });
        let disk_percent = if disk_total > 0 {
            (disk_used as f64 / disk_total as f64 * 100.0) as u16
        } else {
            0
        };
        let disk_used_gb = disk_used / 1024 / 1024 / 1024;
        let disk_total_gb = disk_total / 1024 / 1024 / 1024;

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ])
                .split(f.area());

            let cpu_gauge = Gauge::default()
                .block(Block::default().title(" CPU Usage ").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent(cpu as u16);

            let mem_gauge = Gauge::default()
                .block(Block::default()
                    .title(format!(" RAM Usage  —  {} MB / {} MB ",
                        used_mem / 1024 / 1024,
                        total_mem / 1024 / 1024))
                    .borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(mem_percent);

            let disk_gauge = Gauge::default()
                .block(Block::default()
                    .title(format!(" Disk Usage  —  {} GB / {} GB ", disk_used_gb, disk_total_gb))
                    .borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Yellow))
                .percent(disk_percent);

            f.render_widget(cpu_gauge, chunks[0]);
            f.render_widget(mem_gauge, chunks[1]);
            f.render_widget(disk_gauge, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("Goodbye from syswatch!");
    Ok(())
}