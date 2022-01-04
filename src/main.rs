use std::process::exit;
use anyhow::Result;
use cryptokeep_cli::{ui, dao::Dao};
use console::style;

fn main() -> Result<()> {
    let db_response = Dao::init();

    match db_response {
        Ok(_d) => { log::info!("Application started") }
        Err(e) => {
            println!("Error occurred while initializing database, err: {:?}", e);
            exit(0)
        }
    }
    //
    // console::Term::stdout().clear_screen().expect("");
    // println!("Hello World");
    //
    // std::thread::sleep(std::time::Duration::from_secs(5));
    loop {
        ui::start().expect("Unable to start the application");

        println!("{}", style("\nDo you want to continue ? Y / n\n").cyan());
        let mut buff = String::new();
        std::io::stdin().read_line(&mut buff)?;

        if buff == "N\n" || buff == "n\n" {
            break;
        }
    }

    Ok(())
}
