
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;
use rusqlite::{params};
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

/// Creates a SQLite database if it does not exist 
/// and returns a Pool for connections to this database
pub fn create_db_then_pool(file: &Path) -> Pool {
    let manager = SqliteConnectionManager::file(file);
    let pool = Pool::new(manager).unwrap();
    
    let conn = pool.get().unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, hash TEXT, password BLOB, salt BLOB, data BLOB)", params![]).unwrap();

    pool
}