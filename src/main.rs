extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Spotify CLient")
        .version("0.1")
        .author("Boris Faure <boris.faure@gmail.com>")
        .about("My own dumb spotify client")
        .arg(
            Arg::with_name("CMD")
                .help("Command to run")
                .index(1)
                .possible_values(&["get"])
                .required(true),
        )
        .get_matches();

    println!("cmd: {}", matches.value_of("CMD").unwrap());
}
