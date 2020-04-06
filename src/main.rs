use std::path::PathBuf;
use structopt::StructOpt;

mod identity;
mod manager;

use crate::identity::Identity;
use crate::manager::Manager;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-identity", about = "Manage git identities")]
enum Opt {
    Add {},
    List {
        #[structopt(parse(from_os_str))]
        file: Option<PathBuf>,
    },
    Set {},
}

fn main() {
    let opts = Opt::from_args();

    match opts {
        Opt::List { file } => {
            let file = file.unwrap_or(
                dirs::home_dir()
                    .expect("getting home directory")
                    .join(".gitconfig"),
            );
            let manager = Manager::read_file(file).expect("reading git config file");
            for identity in manager.list_identities() {
                println!("{:?}", identity);
            }
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_identities() {
        let manager = Manager::read_file("fixtures/full").unwrap();
        let identities = manager.list_identities();
        assert_eq!(identities.len(), 2);
    }

    #[test]
    fn test_first_identity() {
        use std::path::PathBuf;

        let manager = Manager::read_file("fixtures/full").unwrap();
        let identities = manager.list_identities();
        assert_eq!(
            identities[0],
            Identity {
                id: "testa".to_string(),
                name: "User Name".to_string(),
                email: "email@example.com".to_string(),
                signing_key: Some("NKA012GKAF".to_string()),
                ssh_key: Some(PathBuf::from("~/.ssh/id_rsa_testa")),
            }
        );
    }

    #[test]
    fn test_second_identity() {
        let manager = Manager::read_file("fixtures/full").unwrap();
        let identities = manager.list_identities();
        assert_eq!(
            identities[1],
            Identity {
                id: "testb".to_string(),
                name: "Another Name".to_string(),
                email: "email2@example.com".to_string(),
                signing_key: Some("HA82LS2LK0".to_string()),
                ssh_key: None,
            }
        );
    }
}
