use std::path::PathBuf;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Identity {
    pub id: String,
    pub name: String,
    pub email: String,
    pub signing_key: Option<String>,
    pub ssh_key: Option<PathBuf>,
}
