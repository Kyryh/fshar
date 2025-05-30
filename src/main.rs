use std::{
    io,
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

use clap::{Arg, Command, ValueHint, builder::styling::Style, value_parser};

mod num_io;

use num_io::*;

const SERVER_SENDING: u8 = 0;
const SERVER_RECEIVING: u8 = 1;

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

fn main() -> io::Result<()> {
    let args = args().get_matches();

    let mode = args.get_one::<String>("mode").unwrap();
    match mode.as_ref() {
        "client" => {
            let addr = (
                args.get_one::<String>("server-address")
                    .expect("Address should be valid")
                    .as_ref(),
                *args
                    .get_one::<u16>("server-port")
                    .expect("Port should be valid"),
            );

            let mut stream = TcpStream::connect(addr).expect("Server should be listening");

            let server_mode = stream.read_num()?;

            match server_mode {
                SERVER_SENDING => receive(stream),
                SERVER_RECEIVING => send(stream),
                _ => unreachable!(),
            }
        }
        s => {
            let port = *args
                .get_one::<u16>("server-port")
                .expect("Port should be valid");
            let listener = TcpListener::bind(("0.0.0.0", port))?;
            let mut stream = listener.accept()?.0;
            let server_mode = &s[7..];
            match server_mode {
                "sender" => {
                    stream.write_num(SERVER_SENDING)?;
                    send(stream)
                }
                "receiver" => {
                    stream.write_num(SERVER_RECEIVING)?;
                    receive(stream)
                }
                _ => unreachable!(),
            }
        }
    }
}

fn send(stream: TcpStream) -> io::Result<()> {
    Ok(())
}

fn receive(stream: TcpStream) -> io::Result<()> {
    Ok(())
}
