pub struct Hls;

impl Hls {
    pub fn stream(packet_arr: &[String; 200], time_arr: [u128; 200]) {
        println!("packet_arr: {:?}, time_arr: {:?}", packet_arr[0], time_arr[0]);
    }
}