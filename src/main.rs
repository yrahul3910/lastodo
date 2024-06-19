mod ui;
mod errors;
mod tui;
mod app;
mod tests;

use std::error::Error;
use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = errors::install_hooks();
    let mut terminal = tui::init()?;
    let _ = App::new().run(&mut terminal);
    tui::restore()?;

    Ok(())
}
