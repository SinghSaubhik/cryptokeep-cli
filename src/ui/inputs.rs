use anyhow::{Error, Result};
use console::Term;
use dialoguer::{Input, Password, Select};
use dialoguer::theme::ColorfulTheme;

pub fn render_select<T: ToString>(items: &Vec<T>) -> Result<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(s) => Ok(s),
        None => Err(Error::msg("Error occurred"))
    }
}

pub fn text_input_prompt(prompt: &str, existing_content: Option<&str>) -> Result<String> {

    let content = existing_content.unwrap_or("");

    let user_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .with_initial_text(content)
        .interact_text()?;
    Ok(user_name)
}

pub fn password_input_prompt(prompt: &str) -> Result<String> {
    let pass = Password::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()?;
    Ok(pass)
}