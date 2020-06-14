use anyhow::Result;
use serde::Serialize;
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new() -> Result<Self> {
        let stream = TcpStream::connect("127.0.0.1:8080")?;
        stream.set_write_timeout(Some(Duration::from_millis(20)))?;

        Ok(Self { stream })
    }

    pub fn send<T: Serialize>(&mut self, body: T) -> Result<()> {
        let mut sended_bytes = serde_json::to_vec(&body)?;
        sended_bytes.extend_from_slice("\n\n".as_bytes());

        self.stream.write(&sended_bytes)?;

        Ok(())
    }
}
