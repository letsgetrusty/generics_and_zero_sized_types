#![allow(unused)]

use std::collections::HashMap;
use std::marker::PhantomData;

struct Locked;
struct Unlocked;

// PasswordManager<Locked> != PasswordManager<Unlocked>

struct PasswordManager<State = Locked> {
    master_pass: String,
    passwords: HashMap<String, String>,
    state: PhantomData<State>,
}

impl PasswordManager<Locked> {
    pub fn unlock(self, master_pass: String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
            state: PhantomData,
        }
    }
}

impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
            state: PhantomData,
        }
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }
}

impl<State> PasswordManager<State> {
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
            state: PhantomData,
        }
    }
}

fn main() {
    let mut manager = PasswordManager::new("password123".to_owned());
    let manager = manager.unlock("password123".to_owned());
    manager.list_passwords();
    manager.lock();
}
