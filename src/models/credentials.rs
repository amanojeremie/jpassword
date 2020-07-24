use serde::{Serialize, Deserialize};

/// Represents a User's credential for another application
#[derive(Serialize, Deserialize)]
pub struct Credential {
    /// The name of the Credential
    pub name: String,
    /// The url of the Credential
    pub url: String,
    /// The username associated with the Credential
    pub username: String,
    /// The password associated with the Credential
    pub password: String
}

#[derive(Serialize, Deserialize)]
/// Represents a list of the User's saved credentials
pub struct Credentials {
    /// A vector of the User's credentials
    pub credentials: Vec<Credential>
}

impl Credentials {
    /// Creates an empty list of Credentials, returning it
    pub fn new() -> Self {
        Credentials {
            credentials: Vec::<Credential>::new()
        }
    }

    /// Adds a new credential in the list of Credentials
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the Credential
    /// * `url` - The url of the Credential
    /// * `username` - The username associated with the Credential
    /// * `password` - The password associated with the Credential
    pub fn create(&mut self, name: String, url: String, username: String, password: String) {
        self.credentials.push(Credential {
            name: name,
            url: url,
            username: username,
            password: password
        })
    }

    /// Updates a Credential in the list of Credentials
    ///
    /// # Arguments
    ///
    /// * `i` - The index of the Credential to update
    /// * `new_cred` - The updated credential to replace the existing with
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

    /// Deletes a Credential in the list of Credentials
    ///
    /// # Arguments
    ///
    /// * `i` - The indexof the Credential to delete
    pub fn delete(&mut self, i: usize) -> Result<(), ()> {
        if i >= self.credentials.len() {
            return Err(());
        }
        self.credentials.remove(i);
        Ok(())
    }

}