
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::{params};
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub fn create_db_then_pool(file: String) -> Pool {
    let manager = SqliteConnectionManager::file(file);
    let pool = Pool::new(manager).unwrap();
    
    let conn = pool.get().unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, hash TEXT, password BLOB, salt BLOB, data BLOB)", params![]).unwrap();

    pool
}