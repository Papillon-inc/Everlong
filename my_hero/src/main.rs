use std::net::UdpSocket;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process;

use my_hero::*;

macro_rules! init_array(
    ($ty:ty, $len:expr, $val:expr) => (
        {
            let mut array: [$ty; $len] = unsafe { std::mem::uninitialized() };
            for i in array.iter_mut() {
                unsafe { ::std::ptr::write(i, $val) }
            }
            array
        }
    )
);

fn main() {
    let address = "127.0.0.1:12345";
    let server_socket = UdpSocket::bind(address)
                           .expect("Could not bind socket");
                    
    println!("UDP Server started!");
    println!("Listening for connections on {}", address);
    
    handle_incoming_packets(server_socket);
}

// Packet contains a name of ts file
fn handle_incoming_packets(socket: UdpSocket) {
    let mut time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(v) => v.as_micros(),
        Err(_) => {
            println!("SystemTime is before UNIX EPOCH!");
            process::exit(0);
        }
    };
    
    let mut packet_arr = init_array!(String, 200, "-1".to_string());
    let mut time_arr = [0u128; 200];
    let mut count = 0;

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

                packet_arr[count] = result_str;
                time_arr[count] = time_diff;
                count += 1;

                if count >= 200 {
                    Hls::stream(&packet_arr, time_arr);
                    count = 0;
                }
            },
            Err(e) => {
                eprintln!("could not receive a datagram: {}", e);
                process::exit(0);
            }
        }
    }
}