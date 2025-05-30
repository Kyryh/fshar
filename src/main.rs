use clap::Command;

fn args() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
}

fn main() {
    let args = args();
    args.get_matches();
}
