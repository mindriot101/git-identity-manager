use crate::identity::Identity;
use anyhow::{bail, Result};
use git2::Config;
use skim::prelude::*;
use std::collections::HashSet;
use std::io::Cursor;
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
    global_config: Config,
    local_config: Option<Config>,
}

impl Manager {
    pub(crate) fn new(path: Option<PathBuf>) -> Result<Self> {
        let global_config_path = Config::find_global()?;
        let global_config = Config::open(&global_config_path)?;

        let local_config = path.map(|p| Config::open(&p).unwrap());

        Ok(Self {
            global_config,
            local_config,
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

    /// Use skim to select an identity interactively
    pub(crate) fn select_identity(&mut self) -> Result<()> {
        let identity_names = self.get_all();
        let input_text = identity_names.join("\n");

        let options = SkimOptions::default();
        let item_reader = SkimItemReader::default();
        let items = item_reader.of_bufread(Cursor::new(input_text));

        let selected_items = Skim::run_with(&options, Some(items))
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());

        if selected_items.is_empty() {
            // Early return, the user did not select anything
            Ok(())
        } else if selected_items.len() > 1 {
            bail!("multiple item selection not supported")
        } else {
            let identity = selected_items[0].text();
            self.use_identity(identity)
        }
    }

    fn use_identity<S>(&mut self, identity_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        let identity_name = identity_name.into();
        let identity = self.get(&identity_name).unwrap();

        if let Some(lconfig) = self.local_config.as_mut() {
            for (key, value) in identity.iter() {
                lconfig.set_str(&key, &value)?;
            }
        } else {
            unreachable!("no local config set");
        };

        Ok(())
    }

    pub(crate) fn remove(&mut self) -> Result<()> {
        match &mut self.local_config {
            Some(config) => {
                let keys_to_remove = config
                    .entries(Some("user.*"))
                    .unwrap()
                    .map(|c| c.unwrap().name().unwrap().to_string())
                    .collect::<Vec<_>>();

                if keys_to_remove.is_empty() {
                    eprintln!("No keys to remove");
                    return Ok(());
                }

                for key in keys_to_remove {
                    config.remove(&key)?;
                }

                Ok(())
            }
            None => unreachable!(),
        }
    }

    pub(crate) fn remove_from_global(&mut self, identity: &str) -> Result<()> {
        let keys_to_remove = self
            .global_config
            .entries(Some(&format!("user.{}.*", identity)))
            .unwrap()
            .map(|c| c.unwrap().name().unwrap().to_string())
            .filter(Self::should_remove_key)
            .collect::<Vec<_>>();
        if keys_to_remove.is_empty() {
            eprintln!("No keys to remove");
            return Ok(());
        }

        for key in keys_to_remove {
            self.global_config.remove(&key)?;
        }
        Ok(())
    }

    /// Check for whether the key should be included when deleting
    ///
    /// We do not want some keys to be exported, for example user.useConfigOnly
    fn should_remove_key(key: &String) -> bool {
        !key.to_lowercase().ends_with("useconfigonly")
    }

    pub(crate) fn list_identities(&self) {
        let identities = self.get_all();
        for profile_name in identities {
            println!("{}", profile_name);
        }
    }

    pub(crate) fn current_identity(&self) -> Option<(String, String)> {
        if let Some(config) = &self.local_config {
            return config
                .get_entry("user.name")
                .and_then(|name_entry| {
                    let name = name_entry.value().unwrap().to_string();
                    config.get_entry("user.email").map(|email_entry| {
                        let email = email_entry.value().unwrap().to_string();
                        (name, email)
                    })
                })
                .map_err(|e| eprintln!("got error: {:?}", e))
                .ok();
        }

        None
    }

    fn get_all(&self) -> Vec<String> {
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

        set.drain().collect()
    }

    fn get(&self, identity: &str) -> Option<Identity> {
        for i in self.get_all() {
            if i == identity {
                let mut iobj = Identity::default();
                iobj.id = identity.to_string();

                // Get the keys
                iobj.name = self
                    .global_config
                    .get_string(&format!("user.{}.name", identity))
                    .unwrap();
                iobj.email = self
                    .global_config
                    .get_string(&format!("user.{}.email", identity))
                    .unwrap();

                if let Ok(signing_key) = self
                    .global_config
                    .get_string(&format!("user.{}.signingkey", identity))
                {
                    iobj.signing_key = Some(signing_key);
                }

                if let Ok(ssh_key) = self
                    .global_config
                    .get_string(&format!("user.{}.sshkey", identity))
                {
                    iobj.ssh_key = Some(PathBuf::from(ssh_key));
                }

                return Some(iobj);
            }
        }

        None
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
