use std::io::ErrorKind;

use bytes::{Buf, BytesMut};
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::resp::Type;

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_message(&mut self) -> Result<Option<Type>, ()> {
        loop {
            println!("Read loop");

            match self.stream.read_buf(&mut self.buffer).await {
                Ok(read) => {
                    println!("Read {} bytes", read);

                    let as_str = std::str::from_utf8(&self.buffer).or(Err(()))?;
                    let res = Type::parse(as_str);

                    if let Some((msg, advance)) = res {
                        self.buffer.advance(advance);
                        return Ok(Some(msg));
                    }

                    if read == 0 {
                        return Ok(None);
                    }
                }

                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => {
                        // Maybe retry once and then break?
                        println!("Here")
                    }

                    _ => return Err(()),
                },
            }
        }
    }
}
