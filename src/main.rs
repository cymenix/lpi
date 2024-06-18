mod app;
mod moon;

use app::App;
use moon::{MOON_REPO, MOON};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{os::unix::process::CommandExt, path::Path};

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
        std::process::Command::new(MOON)
            .current_dir(Path::new(&std::env::var(MOON_REPO).unwrap()))
            .args(["run", &command])
            .exec();
    }

    Ok(())
}
