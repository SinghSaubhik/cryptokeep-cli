use std::io::{stderr, stdout};
use std::process::exit;
use console::Term;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

pub fn draw_option_menu(v: &Vec<&str>) -> Result<usize, std::io::Error> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&v)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();


    Ok(selection.unwrap())
}

pub fn start() -> Result<(), std::io::Error> {
    let term = Term::stdout();

    term.clear_screen();
    let menu_items = vec!["List Secret", "Add New Secret", "Quit"];
    let mut selection = draw_option_menu(&menu_items)?;

    while selection!=2 {
        match selection {
            0 => println!("We will list your secrets"),
            1 => println!("Let's add new secret"),
            2 => exit(0),
            _ => println!("Invalid input")
        }

        term.clear_to_end_of_screen();
        selection = draw_option_menu(&menu_items).unwrap()
    }

    Ok(())
}