use std::{io::{Read, Write}, net::SocketAddr};
use socket2::{Socket, Domain, Type, Protocol};

fn main() -> std::io::Result<()> {
    let mut socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    let server_address: SocketAddr = "127.0.0.1:9990".parse().unwrap();

    socket.connect(&server_address.into())?;

    println!("Conectado ao servidor!");

    socket.write_all(b"Ola, servidor!")?;
    
    let mut buffer = [0_u8; 1024];
    let n = socket.read(&mut buffer)?;

    println!("Resposta do servidor: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
