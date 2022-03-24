use clap::{Arg, Command};

pub fn build() -> Command<'static> {
    Command::new("md5")
        .version("0.1.0")
        .author("Shenggen Liu <gensliu0328@outlook.com>")
        .about("calculate a message-digest fingerprint (checksum) for a file")
        .arg_required_else_help(true)
        .arg(
            Arg::new("string")
                .help("Print a checksum of the given string.")
                .short('s')
                .long("string")
        )
        .arg(
            Arg::new("quiet")
                .help("Quiet mode - only the checksum is printed out.")
                .short('q')
                .long("quiet")
        )
        .arg(
            Arg::new("INPUT")
                .required(true)
                .help("A string or file path.")
        )
}
