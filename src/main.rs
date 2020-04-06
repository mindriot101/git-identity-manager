use structopt::StructOpt;

mod identity;
mod manager;

use crate::manager::Manager;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-identity", about = "Manage git identities")]
enum Opt {
    Add,
    #[structopt(help = "List available identities")]
    List,
    Set,
}

fn main() {
    let manager = Manager::new().unwrap();

    match Opt::from_args() {
        Opt::List => {
            let identities = manager.list_identities();
            for identity in identities {
                println!("{}", identity.id);
            }
        }
        _ => todo!(),
    }
}
