use std::error::Error;
use std::io::Read;
use std::process::exit;
use cryptokeep_cli::{ui, dao::Dao};
use chrono::prelude::*;
use console::style;


fn main() -> Result<(), Box<dyn Error>> {
    let dao: Dao;
    let db_response = Dao::init();

    match db_response {
        Ok(d) => { dao = d }
        Err(e) => {
            println!("Error occurred while initializing database, err: {:?}", e);
            exit(0)
        }
    }

    loop {
        ui::start();

        println!("{}", style("\nDo you want to continue ? Y / n\n").cyan());
        let mut buff = String::new();
        std::io::stdin().read_line(&mut buff)?;

        if buff == "N\n" || buff == "n\n" {
            break;
        }
    }
    Ok(())
}
