extern crate mpeg2ts;

use mpeg2ts::ts::{TsPacketReader, TsPacketWriter, ReadTsPacket, WriteTsPacket};

pub struct Converter;

impl Converter {
    // DEBUG FUNCTION
    pub fn do_print() {
        println!("Sappy");
    }

    pub fn convert() {
        let mut writer = TsPacketWriter::new(std::io::stdout());
        let mut reader = TsPacketReader::new(std::io::stdin());

        while let Some(packet) = Result::unwrap(reader.read_ts_packet()) {
             Result::unwrap(writer.write_ts_packet(&packet));
        }
    }
}