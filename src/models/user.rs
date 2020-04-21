use rusqlite::{Connection, params};
use super::credentials::{Credentials};
use crate::crypto::{aead::{aead_seal, aead_open}, hash::{pbkdf2_rand_salt, pbkdf2_verify, hash}};
use hex;
use std::str;

const PASSWORD_HASH_LEN: usize = 256;

pub struct User {
    id: i64,
    pub username: String,
    pub password: String,
    pub credentials: Credentials
}


impl User {

    pub fn create(conn: &Connection, username: String, password: String) -> Result<Self, ()> {

        if username.contains(":") {
            return Err(());
        }

        let username_hash = hex::encode(hash(username.as_bytes()));

        let mut stmt = conn.prepare("SELECT * FROM users WHERE hash = ?").unwrap();
        if stmt.exists(params![username_hash]).unwrap() {
            return Err(());
        }

        let mut password_hash = [0u8; PASSWORD_HASH_LEN];
        let salt = pbkdf2_rand_salt(password.as_bytes(), &mut password_hash);

        let user_pass_pairs = Credentials::new();
        let user_pass_pairs_json = serde_json::to_string(&user_pass_pairs).unwrap();
        let data = aead_seal(&user_pass_pairs_json.into_bytes(), password.as_bytes());

        if data.is_err() {
            return Err(());
        }

        let mut stmt = conn.prepare("INSERT INTO users (hash, password, salt, data) VALUES (?1, ?2, ?3, ?4)").unwrap();
        let id = stmt.insert(params![username_hash, password_hash.to_vec(), salt, data.unwrap()]);

        if id.is_err() {
            return Err(());
        }
        
        Ok(User {
            id: id.unwrap(),
            username: username,
            password: password,
            credentials: user_pass_pairs
        })
    }

    pub fn login(conn: &Connection, username: String, password: String) -> Result<Self, ()> {
        let username_hash = hex::encode(hash(username.as_bytes()));
        let mut stmt = conn.prepare("SELECT id, hash, password, salt, data FROM users WHERE hash = ?").unwrap();

        let rows = stmt.query(params![username_hash]);

        if rows.is_err() {
            return Err(());
        }
        let mut rows = rows.unwrap();

        let row = rows.next().unwrap();
        if row.is_none() {
            return Err(());
        }

        let row = row.unwrap();

        let (id, password_hash, salt, data): (i64, Vec<u8>, Vec<u8>, Vec<u8>)
            = (row.get(0).unwrap(), row.get(2).unwrap(), row.get(3).unwrap(), row.get(4).unwrap());

        if pbkdf2_verify(password.as_bytes(), &password_hash, &salt).is_err() {
            return Err(());
        }

        let credentials_json = aead_open(&data, password.as_bytes());

        if credentials_json.is_err() {
            return Err(());
        }

        let credentials_json = credentials_json.unwrap();
        let credentials_json = str::from_utf8(&credentials_json).unwrap();
        let credentials: Credentials = serde_json::from_str(credentials_json).unwrap();

        Ok(User {
            id: id,
            username: username,
            password: password,
            credentials: credentials
        })
    }

    pub fn save(&self, conn: &Connection) -> Result<(), ()> {
        let username_hash = hex::encode(hash(self.username.as_bytes()));

        let mut stmt = conn.prepare("SELECT * FROM users WHERE hash = ? AND id <> ?").unwrap();
        if stmt.exists(params![username_hash, self.id]).unwrap() {
            return Err(());
        }

        let mut password_hash = [0u8; PASSWORD_HASH_LEN];
        let salt = pbkdf2_rand_salt(self.password.as_bytes(), &mut password_hash);

        let credentials_json = serde_json::to_string(&self.credentials).unwrap();
        let data = aead_seal(&credentials_json.into_bytes(), self.password.as_bytes());

        let mut stmt = conn.prepare("UPDATE users SET hash = ?, password = ?, salt = ?, data = ? WHERE id = ?").unwrap();
        
        let result = stmt.execute(params![username_hash, password_hash.to_vec(), salt, data.unwrap(), self.id]);

        if result.is_err() {
            return Err(());       
        }
        Ok(())
    }
}