use std::{hint::unreachable_unchecked, path::PathBuf};

use clap::{Arg, Command, ValueHint, builder::styling::Style, value_parser};

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
                .value_parser(value_parser!(u16))
                .default_value("931"),
        )
        .arg(
            Arg::new("input-folder")
                .help(concat!(
                    "Folder to use when sending files, in case:\n",
                    "\u{2022} `mode` is `server-sender`\n",
                    "\u{2022} `mode` is `client` with a `server-receiver` server\n",
                ))
                .value_hint(ValueHint::DirPath)
                .value_parser(value_parser!(PathBuf))
                .default_value("./in"),
        )
        .arg(
            Arg::new("output-folder")
                .help(concat!(
                    "Folder to use when receiving files, in case:\n",
                    "\u{2022} `mode` is `server-receiver`\n",
                    "\u{2022} `mode` is `client` with a `server-sender` server\n",
                ))
                .value_hint(ValueHint::DirPath)
                .value_parser(value_parser!(PathBuf))
                .default_value("./out"),
        )
}

fn main() {
    let args = args().get_matches();

    let mode = args.get_one::<String>("mode").unwrap();
    match mode.as_ref() {
        "client" => {
            todo!()
        }
        s => {
            let server_mode = &s[7..];
            match server_mode {
                "sender" => {
                    todo!()
                }
                "receiver" => {
                    todo!()
                }
                _ => unreachable!(),
            }
        }
    }
}
