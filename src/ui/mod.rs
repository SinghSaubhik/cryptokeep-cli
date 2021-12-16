mod indicator;
mod inputs;
mod terminal;

use console::Term;
use anyhow::Result;

pub use indicator::*;
pub use inputs::*;
pub use terminal::*;

pub fn start() -> Result<()> {
    home_screen::draw();
    Ok(())
}
