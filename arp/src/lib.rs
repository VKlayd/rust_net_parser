#![feature(try_from)]

extern crate libc;
extern crate byteorder;
extern crate ethernet;
extern crate common;
extern crate vlan;
extern crate ethertype;

use byteorder::{ByteOrder, NetworkEndian, BigEndian};
use std::convert::TryFrom;

#[derive(Debug,PartialEq)]
pub struct ArpHdr {
    pub ar_hrd : u16,		/* Format of hardware address.  */
    pub ar_pro : u16,		/* Format of protocol address.  */
    pub ar_hln : u8,		/* Length of hardware address.  */
    pub ar_pln : u8,		/* Length of protocol address.  */
    pub ar_op : u16		/* ARP opcode (command).  */
}

#[derive(Debug,PartialEq)]
pub struct ArpInfo <'a> {
    pub ar_ha: &'a [u8],
    pub addr: &'a [u8]
}

#[derive(Debug,PartialEq)]
pub struct ArpBody <'a> {
    pub source: ArpInfo<'a>,
    pub target : ArpInfo<'a>
}

#[derive(Debug,PartialEq)]
pub struct ArpPacket <'a>{
    pub eth_header: ethernet::EthHeader<'a>,
    pub vlans : Option<Vec<vlan::Vlan>>,
    pub arp_hdr : ArpHdr,
    pub arp_body : ArpBody<'a>
}

#[no_mangle]
pub extern "C" fn arp_recv(port: u32, vid: u16, buf: *mut libc::c_void, len: u32) -> i32 {
    let len: usize = len as usize;
    let mut buf_vector : &[u8];
    unsafe {
        buf_vector = std::slice::from_raw_parts(buf as *mut u8, len);
    }
    let packet = ArpPacket::try_from(buf_vector);
    println!("{:?}", packet);
    0
}

fn parse_arp_header(buf: &[u8]) -> ArpHdr {
    ArpHdr{ ar_hrd: common::u16_parse(&buf[0..2]), ar_pro: common::u16_parse(&buf[2..4]), ar_hln: buf[4], ar_pln: buf[5], ar_op: common::u16_parse(&buf[6..8])}
}

fn parse_arp_body<'a>(typ: u16, buf: &'a[u8]) -> ArpBody {

    let src_ar_ha = &buf[0..6];
    let source_addr = if typ == 0x0800 {
        &buf[6..10]
    } else {
        &buf[6..12]
    };

    let target_ar_ha = if typ == 0x0800 {
        &buf[10..16]
    } else {
        &buf[12..18]
    };

    let target_addr = if typ == 0x0800 {
        &buf[16..20]
    } else {
        &buf[18..24]
    };

    ArpBody{source:ArpInfo{ar_ha:src_ar_ha, addr:source_addr}, target: ArpInfo{ar_ha: target_ar_ha, addr: target_addr}}
}


/*pub trait TryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_from(value: T) -> Result<Self, Self::Error>;
}*/

impl <'a> TryFrom<&'a [u8]> for ArpPacket<'a> {
    type Error = ();
    fn try_from(buf: &'a [u8]) -> Result<Self, ()> {
        let eth_end = 12;

        let (v, ptr) = vlan::parse_vlans(&buf[eth_end..buf.len()]).unwrap();

        let ethtype = ethertype::parse_ethtype(&buf[eth_end + ptr..eth_end + ptr + 2]);
        if ethtype != ethertype::EtherTypes::Arp {
            return Err(());
        }

        let arp_hdr = parse_arp_header(&buf[eth_end + ptr + 2..buf.len()]);

        let arp_body = parse_arp_body(arp_hdr.ar_pro, &buf[eth_end + ptr + 2 + 8..buf.len()]);

        Ok(ArpPacket { eth_header: ethernet::eth_parse(buf), vlans: v, arp_hdr: arp_hdr, arp_body: arp_body })
    }
}

impl <'a> TryFrom<(ethernet::EthHeader<'a>, Option<Vec<vlan::Vlan>>, &'a [u8])> for ArpPacket<'a> {
    type Error = ();
    fn try_from(src: (ethernet::EthHeader<'a>, Option<Vec<vlan::Vlan>>, &'a [u8])) -> Result<ArpPacket, ()> {
        let (eth_header, vlans, payload) = src;
        let arp_hdr = parse_arp_header(&payload[0..payload.len()]);

        let arp_body = parse_arp_body(arp_hdr.ar_pro, &payload[8..payload.len()]);

        Ok(ArpPacket{eth_header, vlans, arp_hdr, arp_body})
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_arp() {
        let mut test_buf = [0x11 as u8,0x12,0x13,0x14,0x15,0x16,0x01,0x02,0x03,0x04,0x05,0x06,0x81,0x00,0x00,0x01,0x08,0x06,0x0,0x1,0x08,0,6,4,0,1,0x01,0x02,0x03,0x04,0x05,0x06,0x1,0x1,0x1,0x1,0,0,0,0,0,0,0x2,0x2,0x2,0x2];
        let res = ArpPacket::try_from(&test_buf[0..test_buf.len()]).unwrap();
        assert_eq!(res, ArpPacket {
            eth_header: ethernet::EthHeader {
                dst_mac: &[17, 18, 19, 20, 21, 22], src_mac: &[1, 2, 3, 4, 5, 6] },
            vlans: Some(vec!(vlan::Vlan{priority_code_point:vlan::ClassesOfService::BE, drop_eligible_indicator:false, vid:1})),
            arp_hdr: ArpHdr {
                ar_hrd: 1, ar_pro: 2048, ar_hln: 6, ar_pln: 4, ar_op: 1 },
            arp_body: ArpBody {
                source: ArpInfo {
                    ar_ha: &[1, 2, 3, 4, 5, 6], addr: &[1, 1, 1, 1] },
                target: ArpInfo {
                    ar_ha: &[0, 0, 0, 0, 0, 0], addr: &[2, 2, 2, 2] } } } );
    }
    #[test]
    #[should_panic(expected="Not ARP")]
    fn check_not_arp() {
        let mut test_buf = [0x11 as u8,0x12,0x13,0x14,0x15,0x16,0x01,0x02,0x03,0x04,0x05,0x06,0x81,0x00,0x00,0x01,0x08,0x08,0x0,0x1,0x08,0,6,4,0,1,0x01,0x02,0x03,0x04,0x05,0x06,0x1,0x1,0x1,0x1,0,0,0,0,0,0,0x2,0x2,0x2,0x2];
        let res = match ArpPacket::try_from(&test_buf[0..test_buf.len()]) {
            Err(_) => panic!("Not ARP"),
            _ => 0
        };
    }
}
