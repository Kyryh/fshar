use std::{
    io,
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

mod command;
mod num_io;
mod receiver;
mod sender;

use clap::ArgMatches;
use num_io::*;

const SERVER_SENDING: u8 = 0;
const SERVER_RECEIVING: u8 = 1;

fn main() -> io::Result<()> {
    let args = command::args().get_matches();

    let mode = args.get_one::<String>("mode").unwrap();
    let result = match mode.as_ref() {
        "client" => client(&args),
        mode => server(&args, mode),
    };
    match result {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            eprintln!("\n{}", err);
            std::process::exit(1)
        }
    }
}

fn client(args: &ArgMatches) -> io::Result<()> {
    let addr = (
        args.get_one::<String>("server-address")
            .expect("Address should be valid")
            .as_ref(),
        *args
            .get_one::<u16>("server-port")
            .expect("Port should be valid"),
    );

    let mut stream = TcpStream::connect(addr)?;

    let server_mode = stream.read_num()?;

    match server_mode {
        SERVER_SENDING => {
            println!("Receiving files from server {}", stream.peer_addr()?);
            receiver::receive(
                stream,
                args.get_one::<PathBuf>("output-folder")
                    .expect("Folder should be valid")
                    .as_ref(),
            )
        }
        SERVER_RECEIVING => {
            println!("Sending files to server {}", stream.peer_addr()?);
            sender::send(
                stream,
                args.get_one::<PathBuf>("input-folder")
                    .expect("Folder should be valid")
                    .as_ref(),
            )
        }
        _ => unreachable!(),
    }
}

fn server(args: &ArgMatches, mode: &str) -> Result<(), io::Error> {
    let port = *args
        .get_one::<u16>("server-port")
        .expect("Port should be valid");
    let listener = TcpListener::bind(("0.0.0.0", port))?;
    for mut stream in listener.incoming().filter_map(Result::ok) {
        let server_mode = &mode[7..];
        match server_mode {
            "sender" => {
                stream.write_num(&SERVER_SENDING)?;
                println!("Sending files to client {}", stream.peer_addr()?);
                sender::send(
                    stream,
                    args.get_one::<PathBuf>("input-folder")
                        .expect("Folder should be valid")
                        .as_ref(),
                )?
            }
            "receiver" => {
                stream.write_num(&SERVER_RECEIVING)?;
                println!("Receiving files from client {}", stream.peer_addr()?);
                receiver::receive(
                    stream,
                    args.get_one::<PathBuf>("output-folder")
                        .expect("Folder should be valid")
                        .as_ref(),
                )?
            }
            _ => unreachable!(),
        }
        if !args.get_flag("keep-listening") {
            break;
        }
    }
    Ok(())
}
