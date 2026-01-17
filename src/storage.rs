use crate::secret::{Secret, SecretError};
use rusqlite::{Connection, Result};

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn new(db_path: &str) -> Result<Self, SecretError> {
        let conn = Connection::open(db_path)
            .map_err(|e| SecretError::StoreErr(format!("Failed to open database: {}", e)))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                key TEXT PRIMARY KEY,
                val TEXT
            )",
            [],
        )
        .map_err(|e| SecretError::StoreErr(format!("Failed to create table: {}", e)))?;

        Ok(Store { conn })
    }

    pub fn save(&self, secret: &Secret) -> Result<(), SecretError> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO secrets (key, val) VALUES (?1, ?2)",
                [secret.key(), secret.val()],
            )
            .map_err(|e| SecretError::StoreErr(format!("Failed to save: {}", e)))?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Secret, SecretError> {
        let mut stmt = self
            .conn
            .prepare("SELECT key, val FROM secrets WHERE key = ?")
            .map_err(|e| SecretError::StoreErr(format!("Failed to prepare statement: {}", e)))?;

        let mut rows = stmt
            .query([key])
            .map_err(|e| SecretError::StoreErr(format!("Failed to query: {}", e)))?;

        let row = rows
            .next()
            .map_err(|e| SecretError::StoreErr(format!("Failed to get row: {}", e)))?
            .ok_or_else(|| SecretError::StoreErr(format!("No secret found for key: {}", key)))?;

        let key: String = row
            .get(0)
            .map_err(|e| SecretError::StoreErr(format!("Failed to get key: {}", e)))?;

        let val: String = row
            .get(1)
            .map_err(|e| SecretError::StoreErr(format!("Failed to get value: {}", e)))?;

        Ok(Secret::new(key, val)?)
    }

    pub fn list_keys(&self) -> Result<Vec<String>, SecretError> {
        let mut stmt = self
            .conn
            .prepare("SELECT key FROM secrets")
            .map_err(|e| SecretError::StoreErr(e.to_string()))?;

        let keys = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| SecretError::StoreErr(e.to_string()))?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| SecretError::StoreErr(e.to_string()))?;

        Ok(keys)
    }

    pub fn delete(&self, key: &str) -> Result<(), SecretError> {
        self.conn
            .execute("DELETE FROM secrets WHERE key = ?", [key])
            .map_err(|e| SecretError::StoreErr(format!("Failed to delete: {}", e)))?;
        Ok(())
    }
}
