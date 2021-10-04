extern crate clap;

use clap::{crate_authors, crate_version, App, SubCommand};

fn main() {
    let matches = App::new("Lib Table Top Interactive (ltti)")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Does awesome things")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
                                         <output> 'Sets an optional output file'
                                         -d... 'Turn debugging information on'",
        )
        .subcommand(
            SubCommand::with_name("tic_tac_toe")
                .about("Play tic-tac-toe in your terminal")
                .arg_from_usage("-l, --list 'lists test values'"),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("d") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }
}
