use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

fn main() {
    let set_command = "*3\r\n$3\r\nSET\r\n$4\r\nasdf\r\n$3\r\njkl\r\n";
    let get_command = "*2\r\n$3\r\nGET\r\n$4\r\nasdf\r\n";

    let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3232);

    let mut stream = TcpStream::connect(address).unwrap();

    stream.write_all(set_command.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.write_all(get_command.as_bytes()).unwrap();
    stream.flush().unwrap();
}
