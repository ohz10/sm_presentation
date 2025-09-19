use std::collections::HashMap;
use std::marker::PhantomData;
use std::path::Path;

struct Locked;
struct Unlocked;

struct Vault<State = Locked> {
    secrets: HashMap<String, String>,
    state: PhantomData<State>,
}

impl Vault<Locked> {
    fn unlock(self, _password: String) -> Vault<Unlocked> {
        // use the master password to decrypt the vault
        Vault {
            secrets: self.secrets,
            state: PhantomData::<Unlocked>,
        }
    }
}

impl Vault<Unlocked> {
    fn lock(self) -> Vault<Locked> {
        Vault {
            secrets: HashMap::<String, String>::new(),
            state: PhantomData::<Locked>,
        }
    }

    fn get_secret(self: &Self, name: String) -> Option<&String> {
        self.secrets.get(&name)
    }

    fn add_secret(self: &mut Self, name: String, value: String) {
        self.secrets.insert(name, value);
    }
}

impl<State> Vault<State> {
    fn encryption(self: &Self) -> String {
        "aes128".into()
    }

    fn version(self: &Self) -> String {
        "0.1.0".into()
    }
}

impl Vault {
    fn new(_path: &Path) -> Vault<Locked> {
        Vault {
            secrets: HashMap::<String, String>::new(),
            state: PhantomData::<Locked> {},
        }
    }
}

fn main() {
    let vault = Vault::new(Path::new("/path/to/vault".into()));
    println!("{}, {}", vault.version(), vault.encryption());

    let mut vault = vault.unlock("password".into());
    vault.add_secret("secret 1".into(), "password".into());

    let secret = vault.get_secret("secret 1".into());
    match secret {
        Some(s) => println!("secret 1: {}", s),
        _ => {}
    }

    println!("{}, {}", vault.version(), vault.encryption());
    vault.lock();
}
