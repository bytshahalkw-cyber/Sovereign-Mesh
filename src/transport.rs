use std::net::UdpSocket;
use std::io;

pub struct TransportLayer {
    socket: UdpSocket,
}

impl TransportLayer {
    pub fn bind(addr: &str) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_broadcast(true)?;
        Ok(Self { socket })
    }

    pub fn send_broadcast(&self, data: &[u8], target: &str) -> io::Result<usize> {
        self.socket.send_to(data, target)
    }

    pub fn receive(&self, buf: &mut [u8]) -> io::Result<(usize, std::net::SocketAddr)> {
        self.socket.recv_from(buf)
    }
}
