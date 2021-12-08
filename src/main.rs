use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use cryptokeep_cli::ui;

fn main() -> Result<(), Box<dyn Error>> {
    ui::start();

    sleep(Duration::from_secs(4));
    Ok(())
}
