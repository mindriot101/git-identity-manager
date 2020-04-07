use crate::identity::Identity;
use anyhow::Result;
use git2::Config;
use std::collections::HashSet;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ConfigKey {
    Name,
    Email,
    SigningKey,
    SshKey,
}

impl std::str::FromStr for ConfigKey {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<ConfigKey, Self::Err> {
        match s {
            "name" => Ok(ConfigKey::Name),
            "email" => Ok(ConfigKey::Email),
            "signingkey" => Ok(ConfigKey::SigningKey),
            "sshkey" => Ok(ConfigKey::SshKey),
            _ => unreachable!(),
        }
    }
}

pub(crate) struct Manager {
    global_config: Config,
    local_config: Option<Config>,
}

impl Manager {
    pub(crate) fn new() -> Result<Self> {
        let global_config_path = Config::find_global()?;
        let global_config = Config::open(&global_config_path)?;

        Ok(Self {
            global_config,
            local_config: None,
        })
    }

    pub(crate) fn use_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let global_config_path = Config::find_global()?;
        let global_config = Config::open(&global_config_path)?;

        let local_config = Config::open(path.as_ref())?;

        Ok(Self {
            global_config,
            local_config: Some(local_config),
        })
    }

    pub(crate) fn add(&mut self, identity: &Identity) {
        self.global_config
            .set_str(&format!("user.{id}.name", id = identity.id), &identity.name)
            .unwrap();
        self.global_config
            .set_str(
                &format!("user.{id}.email", id = identity.id),
                &identity.email,
            )
            .unwrap();
        identity.signing_key.as_ref().map(|key| {
            self.global_config
                .set_str(&format!("user.{id}.signingkey", id = identity.id), key)
                .unwrap()
        });
        identity.ssh_key.as_ref().map(|key| {
            let s = key.as_path().to_str().unwrap();
            self.global_config
                .set_str(&format!("user.{id}.sshkey", id = identity.id), s)
                .unwrap()
        });
    }

    pub(crate) fn use_identity<S>(&mut self, identity_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        let identity_name = identity_name.into();
        let identity = self.get(&identity_name);

        todo!()
    }

    pub(crate) fn remove(&mut self, id: &str) -> Result<()> {
        let key_stub = format!("user.{}.", id);
        let mut keys_to_remove = Vec::new();
        self.identity_keys(|entry| {
            let name = entry.name().unwrap().to_string();

            if name.starts_with(&key_stub) {
                // self.global_config.remove(name)?;
                keys_to_remove.push(name.clone());
            }

            Ok(())
        })?;

        if keys_to_remove.is_empty() {
            eprintln!("no keys to remove");
        }

        for key in keys_to_remove {
            self.global_config.remove(&key)?;
        }

        Ok(())
    }

    pub(crate) fn list_identities(&self) {
        let mut set = HashSet::new();

        self.identity_keys(|entry| {
            let parts = entry.name().unwrap().split(".").collect::<Vec<_>>();
            match parts[parts.len() - 1] {
                "name" | "email" | "signingkey" | "sshkey" => {
                    let tag = parts[1..(parts.len() - 1)].join(".");
                    set.insert(tag);
                    Ok(())
                }
                _ => return Ok(()),
            }
        })
        .unwrap();

        for profile_name in set {
            println!("{}", profile_name);
        }
    }

    fn get(&self, identity: &str) -> Option<Identity> {
        todo!()
    }

    fn identity_keys<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(git2::ConfigEntry) -> Result<()>,
    {
        for entry in &self.global_config.entries(Some("user.*.*.*")).unwrap() {
            let entry = entry.unwrap();
            callback(entry)?;
        }

        Ok(())
    }
}
