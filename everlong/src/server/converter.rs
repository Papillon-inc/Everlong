extern crate mpeg2ts;

use mpeg2ts::ts::{TsPacketReader, TsPacketWriter, ReadTsPacket, WriteTsPacket};

pub struct Converter;

impl Converter {
    pub fn convert() {
        let mut writer = TsPacketWriter::new(std::io::stdout());
        let mut reader = TsPacketReader::new(std::io::stdin());

        // while let Some(packet) = track_try_unwrap!(reader.read_ts_packet()) {
        //     track_try_unwrap!(writer.write_ts_packet(&packet));
        // }
    }
}