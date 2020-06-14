use anyhow::Result;
use serde::Deserialize;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Duration;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new() -> Result<Self> {
        Ok(Self {
            listener: TcpListener::bind("127.0.0.1:8081")?,
        })
    }

    pub fn parse_event<'a, T: Deserialize<'a>>(&mut self, buf: &'a mut Vec<u8>) -> Result<T> {
        let (mut socket, _) = self.listener.accept()?;
        socket.set_read_timeout(Some(Duration::from_millis(20)))?;

        let mut buf_for_read = [0; 128];

        loop {
            match socket.read(&mut buf_for_read) {
                Ok(n) if n > 0 => {
                    socket.flush()?;

                    if &buf_for_read[n - 2..n] == &[10, 10] {
                        buf.extend_from_slice(&buf_for_read[0..n - 2]);
                        break;
                    }

                    buf.extend_from_slice(&buf_for_read[0..n]);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
                _ => {}
            }
        }

        Ok(serde_json::from_slice(buf)?)
    }
}
