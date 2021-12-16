use console::Term;

pub mod home_screen {
    use std::process::exit;
    use console::Term;
    use anyhow::Result;
    use crate::ui::render_select;
    use super::{secrets_screen, mutate_secret};

    fn handle_home_selection(selection: usize) -> Result<()> {
        match selection {
            0 => { secrets_screen::draw(); }
            1 => { mutate_secret::add_new_secret(); }
            2 => {
                println!("Shutting down...");
                exit(0)
            }
            _ => println!("Undefined input")
        }
        Ok(())
    }

    pub fn draw() -> Result<()> {
        let items = vec!["List Secrets", "Add New Secret", "Quit"];

        Term::stdout().clear_screen()?;
        let selection = render_select(&items)?;
        handle_home_selection(selection);
        Ok(())
    }
}

pub mod mutate_secret {
    use console::{Term, style};
    use anyhow::Result;
    use crate::{Dao, Secret};
    use crate::ui::{password_input_prompt, text_input_prompt};

    pub fn add_new_secret() -> Result<()> {
        let mut master_password: String;
        Term::stdout().clear_screen()?;
        loop {
            master_password = password_input_prompt("Enter master password")?;
            if master_password == "123456" {
                break;
            } else {
                println!("{}", style("\nWrong password...try again\n").red());
                continue;
            }
        }

        let title = text_input_prompt("Enter title")?;
        let user_name = text_input_prompt("Enter User Name")?;
        let user_password = password_input_prompt("Enter password")?;

        let secret = Secret::new(
            title.as_str(),
            user_name.as_str(),
            user_password.as_str(),
        );

        let dao = Dao::new()?;
        dao.add_new_secret(&secret)?;

        println!("{}", style("\nSuccessfully added the secret in the DB\n").green());
        Ok(())
    }
}

pub mod secrets_screen {
    use std::process::exit;
    use anyhow::Result;
    use console::{Attribute, Color, style, Term};
    use log::info;
    use crate::{Dao, Secret};
    use crate::ui::render_select;

    fn handle_secret_selection(u: usize, secrets: &Vec<Secret>) {}

    pub fn draw() -> Result<()> {
        let dao = Dao::new()?;
        let items = dao.list_secrets()?;

        Term::stdout().clear_screen()?;
        let selection = render_select(&items)?;
        match selection {
            _ => { draw_secret(&items[selection])? }
        }
        Ok(())
    }

    pub fn draw_secret(secret: &Secret) -> Result<()> {
        let dao = Dao::new()?;
        let items = vec!["Reveal Password", "Copy password", "Update", "Delete", "Quit"];
        Term::stdout().clear_to_end_of_screen()?;


        println!(
            "\nSecret: {}\n",
            style(&secret.title).bright().attr(Attribute::Bold).bg(Color::Blue)
        );


        let s = render_select(&items)?;
        let n = items.len();

        match s {
            _ => if s == n - 1 {
                info!("Shutting down...");
                exit(0);
            } else {
                // println!("\u{1b}[0m\u{1b}[44;38;5;15mCargo.lock\u{1b}[0m");
                println!("{}", style(format!("You have selected: {}", &items[s])).green());
            }
        }

        Ok(())
    }
}