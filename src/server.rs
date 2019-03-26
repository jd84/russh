use std::collections::HashMap;

pub trait Manager {
    type Item;

    fn add(&mut self, item: Self::Item);
}

pub struct Server {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub users: Vec<String>,
    pub group: String,
} 

pub enum Auth {
    PublicKey(String),
    Password,
}

pub struct Account {
    pub name: String,
    pub auth: Auth,
}

pub struct ServerManager {
    servers: HashMap<String, Vec<Server>>,
}

pub struct CredentialManager {
    accounts: Vec<Account>,
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            auth: Auth::Password,
        }
    }

    pub fn with_key(name: &str, key: String) -> Self {
        Self {
            name: String::from(name),
            auth: Auth::PublicKey(key),
        }
    }
}

impl Server {
    pub fn with(name: &str, host: &str, port: u32, users: Vec<String>, group: &str) -> Self {
        Self {
            name: name.to_owned(),
            host: host.to_owned(),
            port,
            users,
            group: group.to_owned(),
        }
    }
}

impl Default for ServerManager {
    fn default() -> Self {
        ServerManager {
            servers: HashMap::new(),
        }
    }
}

impl ServerManager {
    pub fn find(&self, name: &str) -> &Server {
        for (_, servers) in self.servers.iter() {
            if let Some(idx) = servers.iter().position(|r| r.name == name) {
                return servers.get(idx).unwrap();
            }
        }
        panic!("No server found with name `{}`", name);
    }

    pub fn groups(&self) -> Vec<&str> {
        let mut groups = Vec::with_capacity(self.servers.len());
        for (group, _) in self.servers.iter() {
            groups.push(group.as_str());
        }
        groups
    }

    pub fn get_servers(&self, group: &str) -> &Vec<Server> {
        self.servers.get(group).unwrap()
    }
}

impl Manager for ServerManager {
    type Item = Server;

    fn add(&mut self, s: Server) {
        if let Some(storage) = self.servers.get_mut(&s.group) {
            storage.push(s);
        } else {
            self.servers.insert(s.group.clone(), vec![s]);
        }
    }
}

impl Default for CredentialManager {
    fn default() -> Self {
        Self {
            accounts: Vec::new()
        }
    }
}

impl Manager for CredentialManager {
    type Item = Account;

    fn add(&mut self, a: Account) {
        self.accounts.push(a);
    }
}

impl CredentialManager {
    pub fn find(&self, name: &str) -> &Account {
        if let Some(idx) = self.accounts.iter().position(|r| r.name == name) {
            return self.accounts.get(idx).unwrap();
        }
        panic!("No account found for user `{}`", name);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn server_manager_integrity_check() {
        let mut s_manager = ServerManager::default();
        let s = Server::with("test", "test.localhost.local", 22, vec![String::from("root")], "default");
        s_manager.add(s);

        assert_eq!(s_manager.find("test").name, "test");
        assert_eq!(s_manager.groups(), vec!["default"]);
    }

    #[test]
    fn credential_manager_check() {
        let mut c_manager = CredentialManager::default();
        let account = Account::new("root");
        c_manager.add(account);

        assert_eq!(c_manager.find("root").name, "root");
    }
}
