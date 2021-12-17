use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;
use crate::current_date_time;

#[derive(Debug)]
pub struct Secret {
    pub id: String,
    pub title: String,
    pub user_name: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Secret {
    pub fn new(title: &str, user_name: &str, password: &str) -> Self {
        let current_date_time = current_date_time();
        Self {
            id: Uuid::new_v4().to_string(),
            title: String::from(title),
            user_name: String::from(user_name),
            password: String::from(password),
            created_at: current_date_time.clone(),
            updated_at: current_date_time,
        }
    }

    pub fn update(&self, title: String, user_name: String, password: String) -> Self {
        Self {
            id: self.id.clone(),
            title,
            user_name,
            password,
            created_at: self.created_at.clone(),
            updated_at: current_date_time(),
        }
    }
}

impl ToString for Secret {
    fn to_string(&self) -> String {
        self.title.clone()
    }
}


#[derive(Debug)]
pub enum Component {
    Home,
    ListSecrets,
    ViewSingle,
    AddNewSecret,
    UpdateSecret,
    DeleteSecret,
    Unknown,
}

pub struct Response {
    pub component: Component,
    pub selection: usize,
    pub secret: Option<Secret>,
}

impl Response {
    pub fn new(component: Component, selection: usize) -> Self {
        Self {
            component,
            selection,
            secret: None,
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Response { component: Component::Home, selection: 0, secret: None }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Response")
    }
}

impl Debug for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "Response Debug:= Component: {:?},  Selection: {:?}",
            self.component, self.selection).as_str()
        )
    }
}