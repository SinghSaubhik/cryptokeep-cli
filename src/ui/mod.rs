mod indicator;
mod inputs;
mod terminal;

use anyhow::Result;

pub use indicator::*;
pub use inputs::*;
pub use terminal::*;
use crate::{get_component_path, home_dir};
use crate::constants::{APPLICATION_DIR, PASSWORD_FILE_NAME};

pub fn start() -> Result<()> {
    let mut app_path = home_dir();
    app_path.push(APPLICATION_DIR);
    app_path.push(PASSWORD_FILE_NAME);

    println!("{}", app_path.to_str().unwrap());

    // Base application directory does not exists, this is a first run
    if !app_path.exists() {
        onboard_screen::draw().unwrap();
    }

    home_screen::draw().expect("Unable to draw on UI");
    Ok(())
}
