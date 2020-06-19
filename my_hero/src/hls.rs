extern crate hls_m3u8;

//use hls_m3u8::MediaPlaylist;
use std::fs::File;
use std::io::prelude::*;

pub struct Hls;

impl Hls {
    pub fn stream(packet_arr: &[String; 200], time_arr: [u128; 200]) {
        println!("packet_arr: {:?}, time_arr: {:?}", &packet_arr[0], &time_arr[0]);

        // Attempt to stream ts files without indicating byte-range
        let mut m3u8 = String::from("#EXTM3U\n #EXT-X-TARGETDURATION:3\n #EXT-X-VERSION:3\n");
        for i in 0..200 {
            m3u8 = format!("{} #EXTINF: {:?}\n", m3u8, time_arr[i]);
            // Sample directory "../ts/0/{}"
            m3u8 = format!("{}{}.ts\n", m3u8, packet_arr[i]);
        }
        m3u8 = format!("{}#EXT-X-ENDLIST", m3u8);

        println!("m3u8: {}", m3u8);
        //println!("MPL:  {:?}" ,m3u8.parse::<MediaPlaylist>());

        let mut file = File::create("../ts/0/index.m3u8").expect("Failed to create a m3u8 file.");
        match file.write_all(m3u8.as_bytes()) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e)
        }
    }
}