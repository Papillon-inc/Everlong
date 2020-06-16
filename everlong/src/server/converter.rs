extern crate mpeg2ts;
extern crate bytes;

use std::fs;
use std::io::{BufWriter};
use std::path::Path;
use mpeg2ts::ts::{TsPacketWriter, WriteTsPacket};
use mpeg2ts::ts::{TsPacket, TsHeader, AdaptationField, TsPayload};
use mpeg2ts::ts::{Pid, TransportScramblingControl, ContinuityCounter};
use mpeg2ts::ts::payload;
use bytes::Bytes;

pub struct Converter;

impl Converter {
    pub fn convert(data: &Bytes, executed_connection_id: &usize) {
        if !Path::new("../ts").exists() {
            match fs::create_dir("../ts") {
                Err(e) => panic!("ts: {}", e),
                Ok(_) => ()
            };
        }

        let dir_name = format!("../ts/{}", executed_connection_id);

        if !Path::new(&dir_name).exists() {
            match fs::create_dir(&dir_name) {
                Err(e) => panic!("{}: {}", &dir_name, e),
                Ok(_) => ()
            };
        }

        let dir_path = fs::read_dir(dir_name).unwrap();
        let file_name = format!("../ts/{}/{:08?}.ts", executed_connection_id, dir_path.count());

        let mut writer = TsPacketWriter::new(BufWriter::new(fs::File::create(file_name).unwrap()));
        let packets = make_ts_packet(data);
        
        for packet in packets.iter() {
            Result::unwrap(writer.write_ts_packet(&packet));
        }
    }
}

fn make_ts_packet(d: &Bytes) -> Vec<TsPacket> {
    let mut packet: Vec<TsPacket> = Vec::new();
    let mut data = d.clone();
    let payload_size = payload::Bytes::MAX_SIZE - 4 - 2;

    while data.len() > payload_size {
        let bytes = data.slice(0..payload_size);
        data = data.slice(payload_size..);
        
        let adaptation = generate_adaptation_field(&bytes);
        packet.push(adaptation);
    }
    let adaptation = generate_adaptation_field(&data);
    packet.push(adaptation);
    
    packet
}

fn generate_adaptation_field(data: &Bytes) -> TsPacket {
    let header = TsHeader {
        transport_error_indicator: false,
        transport_priority: false,
        pid: Result::unwrap(Pid::new(0)),
        transport_scrambling_control: TransportScramblingControl::NotScrambled,
        continuity_counter: ContinuityCounter::new(),
    };
    let field = AdaptationField {
        discontinuity_indicator: false,
        random_access_indicator: false,
        es_priority_indicator: false,
        pcr: None,
        opcr: None,
        splice_countdown: None,
        transport_private_data: Vec::new(),
        extension: None,
    };
    let mut packet = TsPacket {
        header: header,
        adaptation_field: Some(field),
        payload: None,
    };

    let data_ts_packet = payload::Bytes::new(data).unwrap();
    let payload: Option<TsPayload> = Some(TsPayload::Raw(data_ts_packet));

    packet.payload = payload;

    packet
}