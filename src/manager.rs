use crate::identity::Identity;
use anyhow::Result;
use git2::Config;
use std::collections::HashMap;
use std::path::PathBuf;

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
    identities: Vec<Identity>,
}

impl Manager {
    pub(crate) fn new() -> Result<Self> {
        let global_config_path = Config::find_global().unwrap();
        let global_config = Config::open(&global_config_path).unwrap();

        let mut identity_map = HashMap::new();

        for entry in &global_config.entries(Some("user.*")).unwrap() {
            let entry = entry.unwrap();
            let name = entry.name().unwrap();
            let n_components = name.split(".").count();
            if n_components > 2 {
                let identity_name: String = name
                    .split(".")
                    .skip(1)
                    .take_while(|w| {
                        *w != "name" && *w != "email" && *w != "signingkey" && *w != "sshkey"
                    })
                    .collect::<Vec<_>>()
                    .join(".");

                let value = entry.value().unwrap().to_string();
                let key = name.split(".").last().unwrap().to_string();

                let e = identity_map.entry(identity_name).or_insert(HashMap::new());
                e.insert(key.parse::<ConfigKey>().unwrap(), value);
            }
        }

        let mut identities = Vec::new();

        for (identity_name, details) in &identity_map {
            let mut identity = Identity::default();
            identity.id = identity_name.to_string();
            for (key, value) in details {
                match key {
                    ConfigKey::Name => identity.name = value.to_string(),
                    ConfigKey::Email => identity.email = value.to_string(),
                    ConfigKey::SigningKey => {
                        identity.signing_key = {
                            if value.is_empty() {
                                None
                            } else {
                                Some(value.to_string())
                            }
                        }
                    }
                    ConfigKey::SshKey => identity.ssh_key = Some(PathBuf::from(value)),
                }
            }

            identities.push(identity.clone());
        }

        Ok(Self { identities })
    }

    pub(crate) fn list_identities(&self) -> &[Identity] {
        self.identities.as_ref()
    }
}
