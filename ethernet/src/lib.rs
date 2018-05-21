#![feature(try_from)]
use std::convert::TryFrom;

#[derive(Debug,PartialEq)]
pub struct EthHeader <'a>{
    pub dst_mac: &'a[u8],
    pub src_mac: &'a[u8]
}

impl <'a>TryFrom<&'a [u8]> for EthHeader<'a> {
    type Error = ();
    fn try_from(buf: &'a [u8]) -> Result<EthHeader, ()> {
        if buf.len()>=12 {
            Ok(EthHeader { dst_mac: &buf[0..6], src_mac: &buf[6..12] })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ok() {
        let buf = [11 as u8,22, 33,44,55,66,21,22,23,24,25,26,2];
        let eth = EthHeader::try_from(&buf[..]);
        assert!(eth == Ok(EthHeader{dst_mac: &[11 as u8, 22,33,44,55,66], src_mac: &[21 as u8,22,23,24,25,26]}));
    }
    #[test]
    fn test_too_short() {
        let buf = [11 as u8,22, 33,44,55,66,21,22,23,24,25];
        let eth = EthHeader::try_from(&buf[..]);
        assert!(eth == Err(()));
    }
}