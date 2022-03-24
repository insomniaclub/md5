mod command;
mod core;

use crate::core::Core;
use crate::core::Context;

fn main() {
    let matches = command::build().get_matches_from(wild::args_os());

    // match matches.occurrences_of(id: T)
    let context = Context{ is_file: false, quiet_mode: false };

    Core::new(context, String::new()).checksum().format();
}
