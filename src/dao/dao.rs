use std::path::PathBuf;
use rusqlite::{Connection, params};
use anyhow::Result;
use crate::Secret;
use crate::get_home_dir_path;

const DB_NAME: &'static str = "cryptokeep.db";

fn get_db_path() -> Result<PathBuf> {
    let mut home_dir = get_home_dir_path()?;
    home_dir.push(DB_NAME);
    Ok(home_dir)
}

pub struct Dao {
    connection: Connection,
}


impl Dao {
    pub fn init() -> Result<Self> {
        let dao = Self {
            connection: Connection::open(get_db_path()?)?
        };

        let _res = dao.connection.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                  id              TEXT PRIMARY KEY,
                  title           TEXT NOT NULL,
                  user_name       TEXT NOT NULL,
                  password        TEXT NOT NULL,
                  created_at      TEXT,
                  updated_at      TEXT
                  )",
            [],
        )?;
        Ok(dao)
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            connection: Connection::open(get_db_path()?)?
        })
    }

    pub fn list_secrets(&self) -> Result<Vec<Secret>> {
        let mut stmt = self.connection.prepare("SELECT * FROM secrets")?;
        let secret_iter = stmt.query_map([], |row| {
            Ok(Secret {
                id: row.get("id")?,
                title: row.get("title")?,
                user_name: row.get("user_name")?,
                password: row.get("password")?,
                created_at: row.get("created_at").unwrap_or(String::from("")),
                updated_at: row.get("updated_at").unwrap_or(String::from("")),
            })
        })?;

        let mut result = Vec::new();

        for secret in secret_iter {
            result.push(secret.unwrap());
        }

        Ok(result)
    }

    pub fn add_new_secret(&self, secret: &Secret) -> Result<()> {
        self.connection.execute(
            "INSERT INTO secrets (
                    id, title, user_name, password, created_at, updated_at
                 ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6
                 )",
            params![
                secret.id,
                secret.title,
                secret.user_name,
                secret.password,
                secret.created_at,
                secret.updated_at
            ],
        )?;
        Ok(())
    }

    pub fn update_secret(&self, secret: Secret) -> Result<()> {
        self.connection.execute(
            "UPDATE secrets SET title=?1, user_name=?2, password=?3, updated_at=?4 WHERE id=?5",
            [
                secret.title,
                secret.user_name,
                secret.password,
                secret.updated_at,
                secret.id
            ],
        )?;
        Ok(())
    }

    pub fn delete_secret(&self, id: &str) -> Result<()> {
        self.connection.execute(
            "DELETE FROM secrets WHERE id=?1",
            [id],
        )?;
        Ok(())
    }
}
