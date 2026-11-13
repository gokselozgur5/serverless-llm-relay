use rusqlite::{Connection, Result};

pub struct MemoryStore {
    pub(crate) conn: Connection,
}

impl MemoryStore {
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn })
    }
}
