#![allow(unused)]

use std::collections::HashMap;

// PasswordManager<Locked> != PasswordManager<Unlocked>

pub struct PasswordManager<const LOCKED: bool = true> {
    master_pass: String,
    passwords: HashMap<String, String>,
}

impl PasswordManager<true> {
    pub fn unlock(self, master_pass: String) -> PasswordManager<false> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
        }
    }
}

impl PasswordManager<false> {
    pub fn lock(self) -> PasswordManager<true> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
        }
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }
}

impl<const STATE: bool> PasswordManager<STATE> {
    pub fn encryption(&self) -> String {
        todo!()
    }

    pub fn version(&self) -> String {
        todo!()
    }
}

impl PasswordManager {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_pass,
            passwords: Default::default(),
        }
    }
}

fn main() {
    let mut manager = PasswordManager::new("password123".to_owned());
    let manager = manager.unlock("password123".to_owned());
    manager.list_passwords();
    manager.lock();
}
