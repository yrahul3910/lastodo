mod ui;
mod errors;
mod tui;
mod app;
mod tests;

use std::io::Result;
use app::App;

fn main() -> Result<()> {
    let _ = errors::install_hooks();
    let mut terminal = tui::init()?;
    let _ = App::new().run(&mut terminal);
    tui::restore()?;

    Ok(())
}
