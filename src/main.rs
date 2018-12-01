#[macro_use]
extern crate clap;
extern crate rayon;
extern crate walkdir;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod cli;
mod common;
mod list;

fn main() {
    let app = cli::build_cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("list", Some(l)) => {
            let is_maintainer = l.is_present("maintainer");
            let search = l.value_of("search").unwrap();

            list::list(search, is_maintainer)
        }
        _ => println!("{}", matches.usage()),
    }
}
