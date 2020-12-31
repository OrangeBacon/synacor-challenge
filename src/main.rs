use clap::{App, Arg, SubCommand};

mod solve_door;
mod interpret;

fn main() {
    let matches = App::new("synacor")
        .version("1.0")
        .author("Ella <computer.backup.15@gmail.com>")
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .value_name("input file")
                .help("Binary file to be run"),
        )
        .subcommand(SubCommand::with_name("solve").about("Solve the door puzzle"))
        .get_matches();

    match matches.subcommand() {
        ("file", Some(_)) => solve_door::solve_door(),
        _ => interpret::interpret(
            matches
                .value_of("file")
                .unwrap_or("instructions/challenge.bin"),
        ),
    }
}
