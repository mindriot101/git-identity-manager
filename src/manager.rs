use crate::identity::Identity;
use anyhow::Result;
use std::path::{Path, PathBuf};

pub(crate) struct Manager {
    identities: Vec<Identity>,
}

impl Manager {
    pub(crate) fn list_identities(&self) -> &[Identity] {
        &self.identities
    }
}
