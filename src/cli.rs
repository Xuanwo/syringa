use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    // List related subcommands.
    let list = SubCommand::with_name("list").args(&[
        Arg::with_name("maintainer").short("m").long("maintainer"),
        Arg::with_name("search")
            .takes_value(true)
            .index(1)
            .required(true),
    ]);

    App::new("syringa")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommands(vec![list])
}
