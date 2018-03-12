#[derive(Debug,PartialEq)]
pub struct EthHeader <'a>{
    pub dst_mac: &'a[u8],
    pub src_mac: &'a[u8]
}

pub fn eth_parse <'a> (buf: &'a [u8]) -> EthHeader {
    EthHeader{dst_mac: &buf[0..6], src_mac: &buf[6..12]}
}
