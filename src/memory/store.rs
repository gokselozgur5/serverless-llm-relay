use rusqlite::{params, Connection, Result};

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

    pub fn add_message(&self, user_id: &str, role: &str, content: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO history (user_id, role, content) VALUES (?1, ?2, ?3)",
            params![user_id, role, content],
        )?;
        Ok(())
    }

    pub fn get_history(&self, user_id: &str, limit: usize) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT role, content FROM history WHERE user_id = ?1 ORDER BY id DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![user_id, limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        list.reverse();
        Ok(list)
    }
}
