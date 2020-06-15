use std::net::UdpSocket;
use std::time::Duration;
use std::fs;

pub struct UdpHandler;

impl UdpHandler {
    pub fn handle(_executed_connection_id: &usize) {
        let socket = UdpSocket::bind("127.0.0.1:54321").expect("failed to bind socket");
        socket.set_write_timeout(Some(Duration::from_secs(2))).unwrap();

        let dir_name = format!("../ts/{}", _executed_connection_id);
        let dir_path = fs::read_dir(dir_name).unwrap();
        let packet = format!("{:08?}", dir_path.count());

        socket.send_to(packet.as_bytes(), "127.0.0.1:12345").expect("failed to send data");

        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
    }
}