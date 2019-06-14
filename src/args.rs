use clap::{App, Arg};

pub fn get_matches<'a>() -> clap::ArgMatches<'a> {
    let matches = App::new("russh")
        .version("0.0.5")
        .author("Jan D. <jd84@protonmail.com>")
        .about("russh is a ssh wrapper and connection manager.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .help("The configuration file for russh. Default is `~/.russh/russh.yml`")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List all configured hosts"),
        )
        .arg(
            Arg::with_name("HOST_OR_GROUP")
                .help("The host used for the next connection")
                .index(1)
                .required_unless_one(&["list", "Version", "edit", "status"]),
        )
        .arg(
            Arg::with_name("TO_OR_FROM")
                .help("The source or destination for scp")
                .index(2),
        )
        .arg(
            Arg::with_name("edit")
                .help("Open russh.yml in your favorite editor.")
                .short("e")
                .long("edit"),
        )
        .arg(
            Arg::with_name("status")
                .help("List status for all configured servers.")
                .short("s")
                .long("status"),
        )
        .get_matches();
    matches
}