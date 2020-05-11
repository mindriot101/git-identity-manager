use anyhow::Result;
use std::io::{self, Write};
use std::path::PathBuf;
use structopt::clap::Shell;
use structopt::StructOpt;

mod identity;
mod manager;

use crate::identity::Identity;
use crate::manager::Manager;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-identity", about = "Manage git identities")]
enum Opt {
    /// Add an identity to the global store
    Add {
        #[structopt(short, long, help = "Name of the identity")]
        id: String,
        #[structopt(short, long, help = "Your name")]
        name: String,
        #[structopt(short, long, help = "Your email")]
        email: String,
        #[structopt(short, long, help = "Optional gpg signing key id")]
        signing_key: Option<String>,
        #[structopt(
            short = "S",
            long,
            parse(from_os_str),
            help = "Optional path to SSH key"
        )]
        ssh_key: Option<PathBuf>,
        #[structopt(short, long, help = "Private mode (add identity to private file")]
        private: bool,
    },
    /// List available identities
    List,
    /// Set the identity for the current repository
    Set,
    // TODO: Edit,
    /// Remove an identity from this repository or the global list
    Remove {
        #[structopt(
            short,
            long,
            help = "Act out the removal (nothing happens without this key)"
        )]
        force: bool,
        #[structopt(long, help = "Remove from global identity list")]
        global: bool,
        #[structopt(short, long, help = "Identity name if required")]
        identity: Option<String>,
    },
    /// Get the currently active identity for this repository
    Current,
    /// Generate shell completion
    GenCompletion {
        #[structopt(short, long, help = "Shell to generate completion for")]
        shell: Shell,
    },
}

/// Find the local config file
///
/// Traverse up the filesystem tree until a `.git` directory is found. Then treat the gitconfig
/// found  as the local config file.
fn find_local_config_file() -> Result<Option<PathBuf>> {
    let mut dir = std::env::current_dir()?;
    loop {
        let test_git_config = dir.join(".git").join("config");
        if test_git_config.is_file() {
            return Ok(Some(test_git_config));
        }

        if let Some(newpath) = dir.parent() {
            dir = newpath.to_path_buf();
        } else {
            return Ok(None);
        }
    }
}

fn main() {
    let git_config_file = find_local_config_file().unwrap();
    let mut manager = Manager::new(git_config_file).unwrap();

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
            private,
        } => {
            let identity = Identity {
                id,
                name,
                email,
                signing_key,
                ssh_key,
            };

            manager.add(&identity, private).unwrap();
        }
        Opt::Remove {
            force,
            global,
            identity,
        } => {
            if !force {
                eprintln!("-f/--force not given, no action will be taken");
                return;
            }

            if global {
                match identity {
                    Some(i) => manager.remove_from_global(&i).unwrap(),
                    None => {
                        eprintln!("identity required when removing global identity");
                        return;
                    }
                }
            } else {
                manager.remove().unwrap();
            }
        }

        Opt::Set => {
            manager.select_identity().unwrap();
        }

        Opt::Current => match manager.current_identity() {
            Some((name, email)) => println!("{} ({})", name, email),
            None => println!("none set"),
        },

        Opt::GenCompletion { shell } => {
            let mut app = Opt::clap();
            let mut result = Vec::new();

            app.gen_completions_to("git-identity", shell, &mut result);

            // Print to stdout
            io::stdout().write_all(&result).unwrap();
        }
    }
}
