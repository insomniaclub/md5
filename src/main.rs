mod command;
mod core;

use crate::core::Core;
use crate::core::Context;

// main
fn main() {
    let matches = command::build().get_matches_from(wild::args_os());

    let context = Context{
        is_string: matches.is_present("string"),
        quiet_mode: matches.is_present("quiet"),
    };

    Core::new(context, matches.value_of("INPUT").unwrap().to_string()).checksum().format();
}
