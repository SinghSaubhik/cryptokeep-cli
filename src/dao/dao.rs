use rusqlite::{Connection, Result};

pub fn create_table() -> Result<()> {
    let conn = Connection::open("./test.db")?;

    // println!("{:?}", person_table);

    let res = conn.execute(
        "CREATE TABLE IF NOT EXISTS persons (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )?;

    println!("{}", res);
    Ok(())
}