use crate::identity::Identity;
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) struct Manager {
    identities: Vec<Identity>,
}

impl Manager {
    pub(crate) fn read_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Ok(Self {
            identities: Manager::extract_identities(&contents)?,
        })
    }

    fn extract_identities<S: AsRef<str>>(contents: S) -> Result<Vec<Identity>> {
        let mut result = Vec::new();

        let mut parsing_identity = false;
        let mut current_identity = Identity::default();
        for line in contents.as_ref().lines() {
            let line = line.trim();

            if line.starts_with("[user") {
                if line.starts_with("[user]") {
                    continue;
                }

                let id = line.split("\"").skip(1).next().unwrap();

                if parsing_identity == false {
                    // Start parsing new identity
                    current_identity.id = id.to_string();
                } else {
                    // We must have finished the old identity and have started a new one
                    result.push(current_identity.clone());
                    current_identity = Identity::default();
                    current_identity.id = id.to_string();
                }

                parsing_identity = true;
            } else {
                // Regular line, if we are parsing an identity then add these details to the
                // current identity
                if !parsing_identity {
                    continue;
                }

                if line.starts_with("name") {
                    let words = line.split_whitespace();
                    current_identity.name = words.skip(2).collect::<Vec<_>>().join(" ");
                } else if line.starts_with("email") {
                    let words = line.split_whitespace();
                    current_identity.email = words.skip(2).next().unwrap().to_string();
                } else if line.starts_with("signingkey") {
                    let words = line.split_whitespace();
                    current_identity.signing_key = Some(words.skip(2).next().unwrap().to_string());
                } else if line.starts_with("sshkey") {
                    let words = line.split_whitespace();
                    current_identity.ssh_key =
                        Some(PathBuf::from(words.skip(2).next().unwrap().to_string()));
                }
            }
        }

        // If we have reached the end of file then add the current identity
        result.push(current_identity);

        Ok(result)
    }

    pub(crate) fn list_identities(&self) -> &[Identity] {
        &self.identities
    }
}
