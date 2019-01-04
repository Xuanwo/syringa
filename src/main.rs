#[macro_use]
extern crate clap;
extern crate rayon;
extern crate walkdir;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate git2;
extern crate atty;

mod import;
mod list;
mod common;
mod cli;

fn main() {
    let app = cli::build_cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("list", Some(l)) => {
            let is_maintainer = l.is_present("maintainer");
            let search = l.value_of("search").unwrap();

            list::list(search, is_maintainer)
        }
        ("import", Some(i)) => {
            let aur = i.value_of("aur").unwrap();

            if aur != "" {
                import::import_aur(aur)
            } else {
                unimplemented!()
            }
        }
        _ => println!("{}", matches.usage()),
    }
}
