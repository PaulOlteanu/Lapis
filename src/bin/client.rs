use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

fn main() {
    let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3232);

    let mut stream = TcpStream::connect(address).unwrap();

    let message = "*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n*2\r\n$4\r\naasd\r\n$6\r\nmyrist\r\n";
    stream.write_all(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}
