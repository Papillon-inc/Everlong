extern crate mpeg2ts;
extern crate bytes;

use mpeg2ts::ts::{TsPacketReader, TsPacketWriter, ReadTsPacket, WriteTsPacket};
use mpeg2ts::ts::{AdaptationField, TsPayload};
use bytes::Bytes;

pub struct Converter;

impl Converter {
    pub fn convert(data: &Bytes) {
        println!("Converter is called!");
        // println!("{:?}", *data);

        let mut writer = TsPacketWriter::new(std::io::stdout());

        //Result::unwrap(writer.write_ts_packet(data));
    }

    // This function is necessary for writing ts file.
    pub fn make_ts_packet(data: &Bytes) {
        let f = generate_adaptation_field();
    }
}

fn generate_adaptation_field() {
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
}