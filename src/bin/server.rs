use std::{io::{Read, Write}, net::SocketAddr};
use socket2::{Socket, Domain, Type, Protocol};

fn main() -> std::io::Result<()> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

    let address: SocketAddr = "127.0.0.1:9990".parse().unwrap();

    socket.bind(&address.into())?;

    socket.listen(128)?;

    println!("Servidor escutando na porta 8080...");

    loop {
        let (mut client_socket, client_addr) = socket.accept()?;
        println!("Cliente conectado de: {:?}", client_addr);
        
        let mut buffer = [0_u8; 1024];

        let n = client_socket.read(&mut buffer)?;
        println!("Mensagem recebida: {}", String::from_utf8_lossy(&buffer[..n]));
        
        client_socket.write_all(b"Mensagem recebida!")?;
    }
}
