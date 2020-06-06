extern crate mpeg2ts;
extern crate bytes;

use std::fs;
use std::io::BufWriter;
use std::path::Path;
use mpeg2ts::ts::{TsPacketWriter, WriteTsPacket};
use mpeg2ts::ts::{TsPacket, TsHeader, AdaptationField, TsPayload};
use mpeg2ts::ts::{Pid, TransportScramblingControl, ContinuityCounter};
use mpeg2ts::ts::payload;
use bytes::Bytes;

pub struct Converter;

impl Converter {
    pub fn convert(data: &Bytes, executed_connection_id: &usize) {
        if !Path::new("ts").exists() {
            match fs::create_dir("ts") {
                Err(e) => panic!("ts: {}", e),
                Ok(_) => ()
            };
        }

        let dir_name = format!("ts/{}", executed_connection_id.to_string());

        if !Path::new(&dir_name).exists() {
            match fs::create_dir(&dir_name) {
                Err(e) => panic!("{}: {}", &dir_name, e),
                Ok(_) => ()
            };
        }

        let dir_path = fs::read_dir(dir_name).unwrap();

        let file_name = format!("ts/{}/{}.ts", executed_connection_id.to_string(), dir_path.count().to_string());

        let mut writer = TsPacketWriter::new(BufWriter::new(fs::File::create(file_name).unwrap()));
        let packet = make_ts_packet(data);
        Result::unwrap(writer.write_ts_packet(&packet));
    }
}

// This function is necessary for writing ts file.
fn make_ts_packet(data: &Bytes) -> TsPacket {
    let f = generate_adaptation_field(data);
    f
}

fn generate_adaptation_field(data: &Bytes) -> TsPacket {
    let data_ts_packet = payload::Bytes::new(data).unwrap();
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
    let payload: Option<TsPayload> = Some(TsPayload::Raw(data_ts_packet));

    let packet = TsPacket {
        header: header,
        adaptation_field: Some(field),
        payload: payload,
    };
    packet
}