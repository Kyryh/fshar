use std::hint::unreachable_unchecked;

use clap::{Arg, Command, ValueHint, builder::styling::Style};

fn args() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION").replace(
            "File sharing",
            &format!("{b}F{b:#}ile {b}shar{b:#}ing", b = Style::new().bold()),
        ))
        .arg(
            Arg::new("mode")
                .value_parser(["server-sender", "server-receiver", "client"])
                .help(concat!(
                    "File sharing mode\n",
                    "\u{2022} server-sender: Send all files in folder to the client\n",
                    "\u{2022} server-receiver: Receive all files in client's folder\n",
                    "\u{2022} client: Send/receive files to/from server, depending on server's mode"
                ))
                .hide_possible_values(true)
                .required(true),
        )
        .arg(
            Arg::new("server-address")
                .help("Server's address, required only when `mode` is `client`")
                .required_if_eq("mode", "client")
                .value_hint(ValueHint::Hostname),
        )
        .arg(
            Arg::new("server-port")
                .help(concat!(
                    "Server: port to listen on\n",
                    "Client: port to connect to\n"
                ))
                .default_value("931"),
        )
}

fn main() {
    let args = args().get_matches();

    let mode = args.get_one::<String>("mode").unwrap();
    match mode.as_ref() {
        s if s.starts_with("server-") => {
            let server_mode = &s[7..];
            match server_mode {
                "sender" => {
                    todo!()
                }
                "receiver" => {
                    todo!()
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }
        "client" => {
            todo!()
        }
        _ => unsafe { unreachable_unchecked() },
    }
}
