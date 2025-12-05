use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HistoryEntry {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BookmarkEntry {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub timestamp: DateTime<Utc>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();
        // Ensure directory exists
        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        let conn = Connection::open(db_path)?;
        
        let db = Self { conn };
        db.init_tables()?;
        
        Ok(db)
    }

    fn get_db_path() -> PathBuf {
        // Use XDG data directory or fallback to local folder for development
        let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("./data"));
        path.push("foamium");
        path.push("browser.db");
        path
    }

    fn init_tables(&self) -> Result<()> {
        // History Table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL COLLATE NOCASE,
                title TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        // Bookmarks Table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL UNIQUE COLLATE NOCASE,
                title TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        // Indexes for fast lookups
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_url ON history(url)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bookmarks_url ON bookmarks(url)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_timestamp ON history(timestamp DESC)",
            [],
        )?;

        Ok(())
    }

    // --- History Operations ---

    pub fn add_visit(&self, url: &str, title: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        // Insert new visit
        self.conn.execute(
            "INSERT INTO history (url, title, timestamp) VALUES (?1, ?2, ?3)",
            params![url, title, now],
        )?;

        // Prune old history (keep last 5000 entries)
        // We do this occasionally or on every insert. For simplicity, on every insert is fine for local DB.
        self.conn.execute(
            "DELETE FROM history 
             WHERE id NOT IN (
                 SELECT id FROM history ORDER BY timestamp DESC LIMIT 5000
             )",
            [],
        )?;

        Ok(())
    }

    pub fn get_history(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, url, title, timestamp FROM history ORDER BY timestamp DESC LIMIT ?1"
        )?;
        
        let rows = stmt.query_map([limit], |row| {
            let timestamp_str: String = row.get(3)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(HistoryEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                timestamp,
            })
        })?;

        let mut history = Vec::new();
        for row in rows {
            history.push(row?);
        }
        Ok(history)
    }

    // --- Bookmark Operations ---

    pub fn add_bookmark(&self, url: &str, title: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        // Use INSERT OR REPLACE to update title if URL exists
        self.conn.execute(
            "INSERT OR REPLACE INTO bookmarks (url, title, timestamp) VALUES (?1, ?2, ?3)",
            params![url, title, now],
        )?;
        Ok(())
    }

    pub fn remove_bookmark(&self, url: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM bookmarks WHERE url = ?1",
            params![url],
        )?;
        Ok(())
    }

    pub fn is_bookmarked(&self, url: &str) -> Result<bool> {
        let mut stmt = self.conn.prepare("SELECT count(*) FROM bookmarks WHERE url = ?1")?;
        let count: i64 = stmt.query_row(params![url], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn get_bookmarks(&self) -> Result<Vec<BookmarkEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, url, title, timestamp FROM bookmarks ORDER BY timestamp DESC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            let timestamp_str: String = row.get(3)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            Ok(BookmarkEntry {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                timestamp,
            })
        })?;

        let mut bookmarks = Vec::new();
        for row in rows {
            bookmarks.push(row?);
        }
        Ok(bookmarks)
    }
}
