pub mod home_screen {
    use std::process::exit;
    use console::Term;
    use anyhow::Result;
    use crate::ui::render_select;
    use super::{secrets_screen, mutate_secret};

    fn handle_home_selection(selection: usize) -> Result<()> {
        match selection {
            0 => { secrets_screen::draw()?; }
            1 => { mutate_secret::add_new_secret()?; }
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

        let stdout = Term::stdout();
        stdout.clear_screen()?;
        let selection = render_select(&items)?;
        handle_home_selection(selection)?;
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

        let title = text_input_prompt("Enter title", None)?;
        let user_name = text_input_prompt("Enter User Name", None)?;
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
    use arboard::Clipboard;
    use console::{Attribute, Color, style, Term};
    use log::info;
    use crate::{Dao, Level, Secret, write_color};
    use crate::ui::{password_input_prompt, render_select, text_input_prompt};

    pub fn draw() -> Result<()> {
        let dao = Dao::new()?;
        let mut items = dao.list_secrets()?;

        // TODO: This is a hack, needs to fix later
        let main_menu = Secret::new("Main menu", "", "");
        items.push(main_menu);

        Term::stdout().clear_screen()?;
        let selection = render_select(&items)?;
        match selection {
            _ => if selection == items.len() - 1 {
                return Ok(());
            } else {
                draw_secret(&items[selection])?
            }
        }
        Ok(())
    }

    pub fn draw_secret(secret: &Secret) -> Result<()> {
        // let dao = Dao::new()?;
        let items = vec!["Reveal Password", "Copy password", "Update", "Delete", "Quit"];
        Term::stdout().clear_to_end_of_screen()?;


        println!(
            "\n  Secret: {}\n",
            style(&secret.title).bright().attr(Attribute::Bold).bg(Color::Blue)
        );


        let s = render_select(&items)?;
        let n = items.len();

        match s {
            0 => reveal_secret(secret),
            1 => { copy_password(secret)?; }
            2 => { draw_update_secret(secret)?; }
            3 => { delete_secret(secret)?; }
            _ => if s == n - 1 {
                info!("Shutting down...");
                exit(0);
            }
        }

        Ok(())
    }

    fn reveal_secret(secret: &Secret) {
        write_color(format!("Username: {}", secret.user_name.clone()), Level::BRIGHTBOLD);
        write_color(format!("Password: {}", secret.password.clone()), Level::BRIGHTBOLD);
    }

    fn copy_password(secret: &Secret) -> Result<()> {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(secret.password.clone())?;

        println!("  {}  ", style("Password copied to clipboard").green());
        Ok(())
    }

    fn draw_update_secret(existing_secret: &Secret) -> Result<()> {
        let dao = Dao::new()?;

        let new_title = text_input_prompt(
            "Update your title",
            Some(existing_secret.title.as_str()),
        )?;

        let new_username = text_input_prompt(
            "Update username",
            Some(existing_secret.user_name.as_str()),
        )?;

        let password = password_input_prompt("Enter new password")?;

        let updated_secret = existing_secret.update(
            new_title,
            new_username,
            password,
        );

        dao.update_secret(updated_secret)?;

        write_color("\nSuccessfully updated the secret", Level::SUCCESS);

        Ok(())
    }

    fn delete_secret(secret: &Secret) -> Result<()> {
        let dao = Dao::new()?;
        write_color(
            format!(
                "Please enter the title to confirm and delete: {}",
                style(&secret.title).bold().bright()
            ),
            Level::INFO,
        );

        loop {
            let confirmation_title = text_input_prompt(
                "Enter title to confirm",
                None,
            )?;

            if confirmation_title == secret.title {
                break;
            }
            write_color("\n  Entered title does not match\n", Level::ERROR);
        }

        // Delete here
        dao.delete_secret(secret.id.as_str())?;
        write_color("\n  Successfully deleted...", Level::SUCCESS);

        Ok(())
    }
}

pub mod onboard_screen {
    use anyhow::Result;
    use base64::encode;
    use console::{Term};
    use rand::RngCore;
    use sha2::{Sha512, Digest, Sha256};
    use crate::{EncryptionProvider, get_component_path, Level, write_color};
    use crate::constants::PASSWORD_FILE_NAME;
    use crate::ui::{password_input_prompt, text_input_prompt};

    fn store_encryption_key(password: &str) -> Result<()> {
        let mut key = [0_u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        let encoded = encode(&key);

        // EncryptionProvider::new()

        Ok(())
    }

    fn save_user_info(name: &str, password: &str) -> Result<()> {
        // TODO: Save this name into DB
        println!("{}", name);

        let mut hash = Sha256::new();
        hash.update(password.as_bytes());

        let hashed = hash.finalize();
        let encoded = encode(hashed);
        let comp_path = get_component_path(PASSWORD_FILE_NAME).unwrap();

        std::fs::write(comp_path, encoded).unwrap();

        Ok(())
    }


    pub fn draw() -> Result<()> {
        Term::stdout().clear_screen()?;
        write_color("Welcome to Cryptokeep\n", Level::BRIGHTBOLD);
        write_color(
            "Let's collect some basic info about you, shall we ?\n\n",
            Level::SUCCESS,
        );

        let name = text_input_prompt("Enter your name", None)?;
        let mut user_pass: String;

        loop {
            user_pass = password_input_prompt("Enter master password")?;
            let confirm = password_input_prompt("Re-enter master password")?;

            if user_pass != confirm {
                write_color("Password does not match\n", Level::ERROR);
            }else {
                break
            }
        }

        save_user_info(&name, &user_pass).unwrap();
        println!("{}", name);

        Ok(())
    }
}