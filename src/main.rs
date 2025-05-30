use std::{
    io,
    net::{TcpListener, TcpStream},
};

mod num_io;

use num_io::*;

const SERVER_SENDING: u8 = 0;
const SERVER_RECEIVING: u8 = 1;

mod command;

fn main() -> io::Result<()> {
    let args = command::args().get_matches();

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
