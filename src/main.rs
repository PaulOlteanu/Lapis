use std::{
    borrow::BorrowMut,
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use tokio::net::{TcpListener, TcpStream};

mod command;
mod connection;
mod db;
mod resp;

use crate::db::{Command, Db};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Arc::new(Db::new());

    let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3232);

    println!("Beginning listening on port: {}", address.port());
    let listener = TcpListener::bind(address.to_string()).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await?;

        let db_ref = db.clone();
        tokio::spawn(async move {
            handle_connection(stream, db_ref).await.unwrap();
        });
    }
}

async fn handle_connection(stream: TcpStream, db: Arc<Db>) -> Result<(), ()> {
    let mut connection = connection::Connection::new(stream);

    loop {
        if let Some(msg) = connection.read_message().await? {
            println!("Message: {:?}", msg);
            let command = command::CommandType::try_from(msg);
            println!("Command: {:?}", command);

            if let Ok(command) = command {
                db.run_command(&command);
            }
        } else {
            return Ok(());
        }
    }
}
