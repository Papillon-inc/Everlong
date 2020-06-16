use std::net::UdpSocket;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process;

fn main() {
    let address = "127.0.0.1:12345";
    let server_socket = UdpSocket::bind(address)
                           .expect("Could not bind socket");
                    
    println!("UDP Server started!");
    println!("Listening for connections on {}", address);
    
    handle_incoming_packets(server_socket);
}

fn handle_incoming_packets(socket: UdpSocket) {
    let mut time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v.as_micros(),
        Err(_) => {
            println!("SystemTime is before UNIX EPOCH!");
            process::exit(0);
        }
    };

    loop {
        let mut buf = [0u8; 1024];
        let result: Vec<u8>;
        let client_socket = socket.try_clone().expect("failed to clone socket");

        match socket.recv_from(&mut buf) {
            Ok((number_of_bytes, src)) => {
                result = Vec::from(&buf[0..number_of_bytes]);
                
                client_socket.send_to(&buf, src).expect("failed to send response");

                let display_result = result.clone();
                let result_str = String::from_utf8(display_result).unwrap();
                println!("packet: {:?}", result_str);

                let now =  match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(v) => v.as_micros(),
                    Err(_) => {
                        println!("SystemTime is before UNIX EPOCH!");
                        process::exit(0);
                    }
                };
                let time_diff = now - time;
                time = now;
                println!("time_diff: {:?}", time_diff);
            },
            Err(e) => {
                eprintln!("could not receive a datagram: {}", e);
            }
        }
    }
}