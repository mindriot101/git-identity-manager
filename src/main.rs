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
        #[structopt(short, long)]
        commit: bool,
    },
    #[structopt(help = "List available identities")]
    List,
    Set {
        identity: String,
    },
    Edit,
    Remove,
}

fn main() {
    let mut manager = Manager::new().unwrap();

    match Opt::from_args() {
        Opt::List => {
            let identities = manager.list_identities();
            for identity in identities {
                println!("{}", identity.id);
            }
        }
        Opt::Add {
            id,
            name,
            email,
            signing_key,
            ssh_key,
            commit,
        } => {
            let identity = Identity {
                id,
                name,
                email,
                signing_key,
                ssh_key,
            };

            manager.add(&identity);

            if !commit {
                eprintln!("`-c/--commit` argument not specified, not flushing");
                eprintln!(
                    "identities that would be written: {:#?}",
                    manager.list_identities()
                );
                return;
            }

            manager.flush();
        }
        _ => todo!(),
    }
}