#[macro_use]
extern crate clap;
extern crate lockstep;

use clap::{App, Arg};
use lockstep::Group;

fn main() {
    let matches = App::new("Lockstep Server")
        .arg(
            Arg::with_name("address")
                .help("Address to listen on")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iterations")
                .help("Number of barrier operations to do")
                .default_value("1")
                .takes_value(true),
        )
        .get_matches();

    let mut group = Group::new_client(matches.value_of("address").unwrap()).unwrap();
    for _ in 0..value_t_or_exit!(matches, "iterations", usize) {
        group.barrier();
    }
}
