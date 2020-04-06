use std::path::PathBuf;

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct Identity {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) signing_key: Option<String>,
    pub(crate) ssh_key: Option<PathBuf>,
}
