use std::net::UdpSocket;
use std::thread;

fn main() {
    let port = "127.0.0.1:12345";
    let server_socket = UdpSocket::bind(port)
                           .expect("Could not bind socket");
                    
    println!("UDP Server started!");
    println!("Listening for connections on {}", port);
    
    loop {
        let mut buf = [0u8; 1024];
        let mut result: Vec<u8> = Vec::new();
        let client_socket = server_socket.try_clone().expect("failed to clone socket");

        match server_socket.recv_from(&mut buf) {
            Ok((number_of_bytes, src)) => {
                thread::spawn(move || {
                    result = Vec::from(&buf[0..number_of_bytes]);
                    
                    client_socket.send_to(&buf, src).expect("failed to send response");

                    let display_result = result.clone();
                    let result_str = String::from_utf8(display_result).unwrap();
                    println!("{:?}", result_str);
                });
            },
            Err(e) => {
                eprintln!("could not receive a datagram: {}", e);
            }
        }
    }
}