use anyhow::Result;
use rusqlite::{Connection, params};

pub struct FileDB {
    conn: Connection,
}

fn create_table_if_not_exists(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS downloads (
            id INTEGER PRIMARY KEY,
            filename TEXT NOT NULL,
            race_name TEXT
        )",
        [],
    )?;
    Ok(())
}

impl FileDB {
    pub fn new() -> Self {
        let conn = Connection::open("downloads.db").unwrap();
        create_table_if_not_exists(&conn).unwrap();
        FileDB { conn }
    }

    pub fn check_entry_exists(&self, file_name: &str, race_name: &str) -> bool {
        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM downloads WHERE filename = ?1 AND race_name = ?2")
            .unwrap();
        let count: i32 = stmt
            .query_row([file_name, race_name], |row| row.get(0))
            .unwrap();
    
        count > 0
    }

    pub fn new_entry(&self, file_name: &str, race_name: &str) -> Result<()> {
        if self.check_entry_exists(file_name, race_name) {
            return Ok(());
        }
    
        self.conn.execute(
            "INSERT INTO downloads (filename, race_name) VALUES (?1, ?2)",
            params![file_name.to_string(), race_name.to_string()],
        )?;
        Ok(())
    }
    
}
