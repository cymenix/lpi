mod app;
mod moon;

use app::App;
use moon::{MOON, MOON_REPO};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{path::Path, process::Command};

fn main() -> std::io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();

    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new();
    let res = app.run(&mut terminal);

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(ref err) = res {
        println!("{err:?}");
    }

    if let Ok(Some(command)) = res {
        let mut child = Command::new(MOON)
            .current_dir(Path::new(&std::env::var(MOON_REPO).unwrap()))
            .args(["run", &command])
            .spawn()?;

        let status = child.wait()?;

        if !status.success() {
            eprintln!("Command executed with a non-zero status");
        }
    }

    Ok(())
}
