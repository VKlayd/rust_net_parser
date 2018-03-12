extern crate byteorder;
use byteorder::{ByteOrder, NetworkEndian, BigEndian};

pub fn u16_parse (buf: &[u8]) -> u16 {
    NetworkEndian::read_u16(&buf[0..2])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
