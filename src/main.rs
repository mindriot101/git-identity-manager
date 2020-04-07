use anyhow::{bail, Result};
use std::path::PathBuf;
use structopt::StructOpt;

mod identity;
mod manager;

use crate::identity::Identity;
use crate::manager::Manager;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-identity", about = "Manage git identities")]
enum Opt {
    Add {
        #[structopt(short, long)]
        id: String,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        email: String,
        #[structopt(short, long)]
        signing_key: Option<String>,
        #[structopt(short = "S", long, parse(from_os_str))]
        ssh_key: Option<PathBuf>,
    },
    #[structopt(help = "List available identities")]
    List,
    Set {
        identity: String,
    },
    Edit,
    Remove {
        id: String,
        #[structopt(short, long)]
        force: bool,
    },
}

/// Find the local config file
///
/// Traverse up the filesystem tree until a `.git` directory is found. Then treat the gitconfig
/// found  as the local config file.
fn find_local_config_file() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        let test_git_config = dir.join(".git").join("config");
        if test_git_config.is_file() {
            return Ok(test_git_config);
        }

        if let Some(newpath) = dir.parent() {
            dir = newpath.to_path_buf();
        } else {
            bail!("cannot find .git directory");
        }
    }
}

fn main() {
    let mut manager = Manager::new().unwrap();

    match Opt::from_args() {
        Opt::List => {
            manager.list_identities();
        }
        Opt::Add {
            id,
            name,
            email,
            signing_key,
            ssh_key,
        } => {
            let identity = Identity {
                id,
                name,
                email,
                signing_key,
                ssh_key,
            };

            manager.add(&identity);
        }
        Opt::Remove { id, force } => {
            if !force {
                eprintln!("-f/--force not given, no action will be taken");
                return;
            }

            manager.remove(&id).unwrap();
        }

        Opt::Set { identity } => {
            let git_config_file = find_local_config_file().unwrap();
            let mut manager = Manager::use_file(git_config_file).unwrap();
            manager.use_identity(identity).unwrap();
        }

        _ => todo!(),
    }
}
