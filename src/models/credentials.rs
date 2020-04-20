use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub credentials: Vec<Credential>
}

impl Credentials {
    pub fn new() -> Self {
        Credentials {
            credentials: Vec::<Credential>::new()
        }
    }

    pub fn create(&mut self, name: String, url: String, username: String, password: String) {
        self.credentials.push(Credential {
            name: name,
            url: url,
            username: username,
            password: password
        })
    }

    pub fn update(&mut self, i: usize, new_cred: Credential) -> Result<(), ()> {
        if i >= self.credentials.len() {
            return Err(());
        }

        let mut credential = self.credentials.get_mut(i).unwrap();
        credential.name = new_cred.name;
        credential.url = new_cred.url;
        credential.username = new_cred.username;
        credential.password = new_cred.password;

        Ok(())
    }

    pub fn delete(&mut self, i: usize) -> Result<(), ()> {
        if i >= self.credentials.len() {
            return Err(());
        }
        self.credentials.remove(i);
        Ok(())
    }

}