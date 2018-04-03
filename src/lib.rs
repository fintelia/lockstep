extern crate hostname;

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub enum Group {
    Server(Vec<TcpStream>),
    Client(TcpStream),
}
impl Group {
    pub fn new_server<A: ToSocketAddrs>(num_clients: usize, addr: A) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let mut clients = Vec::new();
        for _ in 0..num_clients {
            clients.push(listener.accept()?.0);
        }

        Ok(Group::Server(clients))
    }

    pub fn new_client<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Group::Client(TcpStream::connect(addr)?))
    }

    pub fn from_hostname(leader: &str, port: u16, num_clients: usize) -> io::Result<Self> {
        if leader == hostname::get_hostname().unwrap() {
            Self::new_server(num_clients, ("0.0.0.0", port))
        } else {
            Self::new_client((leader, port))
        }
    }

    pub fn barrier(&mut self) {
        let mut buf = [0; 1];

        match *self {
            Group::Server(ref mut clients) => {
                for c in clients.iter_mut() {
                    c.read_exact(&mut buf).unwrap();
                }
                buf[0] = 0;
                for c in clients {
                    c.write_all(&buf).unwrap();
                }
            }
            Group::Client(ref mut stream) => {
                stream.write_all(&buf).unwrap();
                stream.read_exact(&mut buf).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn it_works() {
        let t1 = thread::spawn(move || Group::new_client("127.0.0.1:10000").unwrap().barrier());
        let t2 = thread::spawn(move || Group::new_client("127.0.0.1:10000").unwrap().barrier());
        let mut s = Group::new_server(2, "127.0.0.1:10000").unwrap();
        s.barrier();

        t1.join().unwrap();
        t2.join().unwrap();
    }
}
