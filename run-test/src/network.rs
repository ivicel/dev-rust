#![allow(dead_code)]

use core::time;
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    thread,
};

pub fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }

        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}

#[cfg(test)]
mod test_tcp {
    use crate::network::handle_connection;
    use std::{
        io::{self, BufRead, BufReader, Write},
        net::{TcpListener, TcpStream},
        thread::{self, JoinHandle},
    };

    #[test]
    fn start_server() -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        let mut thrs: Vec<JoinHandle<()>> = Vec::new();

        for stream in listener.incoming() {
            let stream = stream.expect("connect failed");
            let handler = thread::spawn(move || {
                handle_connection(stream).unwrap_or_else(|err| println!("handle connect: {}", err));
            });
            thrs.push(handler);
        }

        for thr in thrs {
            thr.join().unwrap();
        }

        Ok(())
    }

    #[test]
    fn start_client() -> io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        for _ in 0..10 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read");
            stream.write(input.as_bytes()).expect("failed to write");

            let mut reader = BufReader::new(&stream);
            let mut buffer = Vec::new();
            reader
                .read_until(b'\n', &mut buffer)
                .expect("failed to read into buffer");
            println!("{}", std::str::from_utf8(&buffer).unwrap())
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_udp {
    use std::{io, net::UdpSocket};

    #[test]
    fn udp_server() {
        let socket = UdpSocket::bind("127.0.0.1:8090").expect("bind failed");
        loop {
            let mut buf = [0u8; 512];
            let (bytes, sock) = socket.recv_from(&mut buf).expect("read msg failed");
            println!(
                "receive from {}: {}",
                sock.ip(),
                std::str::from_utf8(&buf[..bytes]).unwrap()
            );
            buf.reverse();
            socket.send_to(&buf, sock).expect("send msg failed");
        }
    }

    #[test]
    fn udp_client() {
        let socket = UdpSocket::bind("127.0.0.1:10080").expect("bind failed");
        // socket.connect("127.0.0.1:8090").expect("connect failed");
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("read error");

            socket
                .send_to(input.as_bytes(), "127.0.0.1:8090")
                .expect("send error");
            let mut buf = [0u8; 512];
            socket.recv(&mut buf).expect("read from server error");
            println!("read from server: {}", std::str::from_utf8(&buf).unwrap());
        }
    }
}

#[cfg(test)]
mod test_addr {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_ip_addr() {
        let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

        assert_eq!("127.0.0.1".parse(), Ok(v4));
        assert_eq!("::1".parse(), Ok(v6));

        println!("{}", v4.is_ipv4());
    }
}
