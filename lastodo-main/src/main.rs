mod actions;
mod app;
mod errors;
mod tui;
mod ui;

use app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = errors::install_hooks();
    let mut terminal = tui::init()?;
    let _ = App::new().run(&mut terminal);
    tui::restore()?;

    Ok(())
}
