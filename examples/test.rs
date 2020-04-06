use git2::Config;
use std::path::Path;

fn main() {
    let global_config_path = Config::find_global().unwrap();
    let global_config = Config::open(&global_config_path).unwrap();
    for entry in &global_config.entries(None).unwrap() {
        let entry = entry.unwrap();
        println!("{} -> {}", entry.name().unwrap(), entry.value().unwrap());
    }

    println!("getting local config");
    let local_config = Config::open(Path::new(".git/config")).unwrap();
    for entry in &local_config.entries(None).unwrap() {
        let entry = entry.unwrap();
        println!("{} -> {}", entry.name().unwrap(), entry.value().unwrap());
    }
}
