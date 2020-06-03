extern crate mpeg2ts;
extern crate bytes;

use std::fs;
use std::io::BufWriter;
use mpeg2ts::ts::{TsPacketReader, TsPacketWriter, ReadTsPacket, WriteTsPacket};
use mpeg2ts::ts::{TsPacket, TsHeader, AdaptationField, TsPayload};
use mpeg2ts::ts::{Pid, TransportScramblingControl, ContinuityCounter};
use bytes::Bytes;

pub struct Converter {
    count: u64,
}

impl Converter {
    pub fn convert(data: &Bytes) {
        let mut writer = TsPacketWriter::new(BufWriter::new(fs::File::create("ts/one.ts").unwrap()));
        let packet = make_ts_packet(data);
        Result::unwrap(writer.write_ts_packet(&packet));
    }
}

// This function is necessary for writing ts file.
fn make_ts_packet(data: &Bytes) -> TsPacket {
    let f = generate_adaptation_field();
    f
}

fn generate_adaptation_field() -> TsPacket {
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
    let payload: Option<TsPayload> = None;

    let packet = TsPacket {
        header: header,
        adaptation_field: Some(field),
        payload: payload,
    };
    packet
}