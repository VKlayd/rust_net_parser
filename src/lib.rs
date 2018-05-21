#![feature(try_from)]

extern crate common;
extern crate ethernet;
extern crate ethertype;
extern crate vlan;
extern crate arp;
extern crate ip;

use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
enum Packet<'a> {
    Arp (arp::ArpPacket<'a>),
    IpV4 (ip::IpV4Packet<'a>),
    Unknown { ethernet: ethernet::EthHeader<'a>, vlans: Option<Vec<vlan::Vlan>>, ethertype: ethertype::EtherType },
}

fn parse_packet<'a>(buf: &'a [u8]) -> Result<Packet, ()> {
    const ETH_END: usize = 12;
    let ethernet = ethernet::EthHeader::try_from(buf)?;
    let (v, ptr) = vlan::parse_vlans(&buf[ETH_END..buf.len()]).unwrap();

    match ethertype::parse_ethtype(&buf[ETH_END + ptr..ETH_END + ptr + 2]) {
        ethertype::EtherTypes::Arp => {
            Ok(Packet::Arp(arp::ArpPacket::try_from((ethernet, v, &buf[ETH_END + ptr + 2 .. buf.len()]))?))
        }
        ethertype::EtherTypes::Ipv4 => {
            Ok(Packet::IpV4(ip::IpV4Packet::try_from((ethernet, v, &buf[ETH_END + ptr + 2 .. buf.len()]))?))
        }
        eth => {
            Ok(Packet::Unknown { ethernet, vlans: v, ethertype: eth })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_arp() {
        let test_buf = [0x11 as u8, 0x12, 0x13, 0x14, 0x15, 0x16, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x81, 0x00, 0x00, 0x01, 0x08, 0x06, 0x0, 0x1, 0x08, 0, 6, 4, 0, 1, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x1, 0x1, 0x1, 0x1, 0, 0, 0, 0, 0, 0, 0x2, 0x2, 0x2, 0x2];
        let res = parse_packet(&test_buf).unwrap();
        assert_eq!(res, Packet::Arp (arp::ArpPacket{
            eth_header: ethernet::EthHeader {
                dst_mac: &[17, 18, 19, 20, 21, 22],
                src_mac: &[1, 2, 3, 4, 5, 6],
            },
            vlans: Some(vec!(vlan::Vlan{drop_eligible_indicator: false, priority_code_point:vlan::ClassesOfService::BE, vid:1})),
            arp_hdr: arp::ArpHdr {
                ar_hrd: 1,
                ar_pro: 2048,
                ar_hln: 6,
                ar_pln: 4,
                ar_op: 1,
            },
            arp_body: arp::ArpBody {
                source: arp::ArpInfo {
                    ar_ha: &[1, 2, 3, 4, 5, 6],
                    addr: &[1, 1, 1, 1],
                },
                target: arp::ArpInfo {
                    ar_ha: &[0, 0, 0, 0, 0, 0],
                    addr: &[2, 2, 2, 2],
                },
            },
        }));
    }

    #[test]
    fn check_not_arp() {
        let test_buf = [0x11 as u8, 0x12, 0x13, 0x14, 0x15, 0x16, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x81, 0x00, 0x00, 0x01, 0x08, 0x08, 0x0, 0x1, 0x08, 0, 6, 4, 0, 1, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x1, 0x1, 0x1, 0x1, 0, 0, 0, 0, 0, 0, 0x2, 0x2, 0x2, 0x2];
        let res = parse_packet(&test_buf).unwrap();
        assert_eq!(res, Packet::Unknown {
            ethernet: ethernet::EthHeader {
                dst_mac: &[17, 18, 19, 20, 21, 22],
                src_mac: &[1, 2, 3, 4, 5, 6],
            },
            vlans: Some(vec!(vlan::Vlan{drop_eligible_indicator: false, priority_code_point:vlan::ClassesOfService::BE, vid:1})),
            ethertype: ethertype::EtherType(0x0808),
        });
    }

    #[test]
    fn check_ip() {
        let test_buf = [0xd0 as u8, 0x17, 0xc2, 0x97, 0x48, 0xb2, 0xa8, 0xf9, 0x4b, 0x80, 0xe7, 0x00, 0x08, 0x00, 0x45, 0x00,
        0x05, 0xbf, 0xe5, 0x46, 0x40, 0x00, 0x3e, 0x06, 0xce, 0x9f, 0xac, 0x10, 0x00, 0x53, 0xc0, 0xa8,
        0x16, 0x47, 0x1f, 0x90, 0xaa, 0x5c, 0xa2, 0x1f, 0x25, 0xd1, 0xf5, 0x43, 0x81, 0x57, 0x80, 0x18,
        0x00, 0xf9, 0x70, 0x6d, 0x00, 0x00, 0x01, 0x01, 0x08, 0x0a, 0x32, 0x87, 0xea, 0x70, 0x62, 0x2f,
        0x45, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x4d, 0x69, 0x6c, 0x6c, 0x69,
        0x73, 0x22, 0x3a, 0x31, 0x35, 0x32, 0x36, 0x36, 0x33, 0x31, 0x37, 0x34, 0x36, 0x33, 0x38, 0x38,
        0x2c, 0x22, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73,
        0x22, 0x3a, 0x37, 0x36, 0x35, 0x37, 0x2c, 0x22, 0x70, 0x61, 0x75, 0x73, 0x65, 0x44, 0x75, 0x72,
        0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a, 0x30, 0x2c, 0x22,
        0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x22, 0x3a, 0x5b, 0x22, 0x33,
        0x35, 0x22, 0x5d, 0x7d, 0x5d, 0x7d, 0x2c, 0x7b, 0x22, 0x5f, 0x6c, 0x69, 0x6e, 0x6b, 0x73, 0x22,
        0x3a, 0x7b, 0x22, 0x73, 0x65, 0x6c, 0x66, 0x22, 0x3a, 0x7b, 0x22, 0x68, 0x72, 0x65, 0x66, 0x22,
        0x3a, 0x22, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x47, 0x50, 0x4f, 0x4e, 0x2f, 0x6a, 0x6f, 0x62, 0x2f,
        0x4c, 0x54, 0x50, 0x2d, 0x58, 0x2d, 0x67, 0x69, 0x74, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x4c, 0x54,
        0x50, 0x2d, 0x38, 0x58, 0x2d, 0x52, 0x45, 0x56, 0x43, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x6d, 0x61,
        0x73, 0x74, 0x65, 0x72, 0x2f, 0x32, 0x36, 0x36, 0x2f, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69,
        0x6f, 0x6e, 0x2f, 0x6e, 0x6f, 0x64, 0x65, 0x2f, 0x34, 0x32, 0x2f, 0x77, 0x66, 0x61, 0x70, 0x69,
        0x2f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x22, 0x7d, 0x7d, 0x2c, 0x22, 0x69, 0x64,
        0x22, 0x3a, 0x22, 0x34, 0x32, 0x22, 0x2c, 0x22, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x3a, 0x22, 0x50,
        0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x22, 0x2c, 0x22, 0x65, 0x78, 0x65, 0x63, 0x4e, 0x6f, 0x64,
        0x65, 0x22, 0x3a, 0x22, 0x22, 0x2c, 0x22, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x3a, 0x22,
        0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x22, 0x2c, 0x22, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54,
        0x69, 0x6d, 0x65, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a, 0x31, 0x35, 0x32, 0x36, 0x36,
        0x33, 0x31, 0x37, 0x35, 0x34, 0x30, 0x36, 0x34, 0x2c, 0x22, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69,
        0x6f, 0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a, 0x33, 0x34, 0x39, 0x30, 0x2c, 0x22,
        0x70, 0x61, 0x75, 0x73, 0x65, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x69, 0x6c,
        0x6c, 0x69, 0x73, 0x22, 0x3a, 0x30, 0x2c, 0x22, 0x73, 0x74, 0x61, 0x67, 0x65, 0x46, 0x6c, 0x6f,
        0x77, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x22, 0x3a, 0x5b, 0x7b, 0x22, 0x5f, 0x6c, 0x69, 0x6e, 0x6b,
        0x73, 0x22, 0x3a, 0x7b, 0x22, 0x73, 0x65, 0x6c, 0x66, 0x22, 0x3a, 0x7b, 0x22, 0x68, 0x72, 0x65,
        0x66, 0x22, 0x3a, 0x22, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x47, 0x50, 0x4f, 0x4e, 0x2f, 0x6a, 0x6f,
        0x62, 0x2f, 0x4c, 0x54, 0x50, 0x2d, 0x58, 0x2d, 0x67, 0x69, 0x74, 0x2f, 0x6a, 0x6f, 0x62, 0x2f,
        0x4c, 0x54, 0x50, 0x2d, 0x38, 0x58, 0x2d, 0x52, 0x45, 0x56, 0x43, 0x2f, 0x6a, 0x6f, 0x62, 0x2f,
        0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2f, 0x32, 0x36, 0x36, 0x2f, 0x65, 0x78, 0x65, 0x63, 0x75,
        0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x6e, 0x6f, 0x64, 0x65, 0x2f, 0x34, 0x33, 0x2f, 0x77, 0x66, 0x61,
        0x70, 0x69, 0x2f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x22, 0x7d, 0x2c, 0x22, 0x6c,
        0x6f, 0x67, 0x22, 0x3a, 0x7b, 0x22, 0x68, 0x72, 0x65, 0x66, 0x22, 0x3a, 0x22, 0x2f, 0x6a, 0x6f,
        0x62, 0x2f, 0x47, 0x50, 0x4f, 0x4e, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x4c, 0x54, 0x50, 0x2d, 0x58,
        0x2d, 0x67, 0x69, 0x74, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x4c, 0x54, 0x50, 0x2d, 0x38, 0x58, 0x2d,
        0x52, 0x45, 0x56, 0x43, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2f,
        0x32, 0x36, 0x36, 0x2f, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x6e, 0x6f,
        0x64, 0x65, 0x2f, 0x34, 0x33, 0x2f, 0x77, 0x66, 0x61, 0x70, 0x69, 0x2f, 0x6c, 0x6f, 0x67, 0x22,
        0x7d, 0x7d, 0x2c, 0x22, 0x69, 0x64, 0x22, 0x3a, 0x22, 0x34, 0x33, 0x22, 0x2c, 0x22, 0x6e, 0x61,
        0x6d, 0x65, 0x22, 0x3a, 0x22, 0x53, 0x68, 0x65, 0x6c, 0x6c, 0x20, 0x53, 0x63, 0x72, 0x69, 0x70,
        0x74, 0x22, 0x2c, 0x22, 0x65, 0x78, 0x65, 0x63, 0x4e, 0x6f, 0x64, 0x65, 0x22, 0x3a, 0x22, 0x22,
        0x2c, 0x22, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x3a, 0x22, 0x53, 0x55, 0x43, 0x43, 0x45,
        0x53, 0x53, 0x22, 0x2c, 0x22, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x4d, 0x69,
        0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a, 0x31, 0x35, 0x32, 0x36, 0x36, 0x33, 0x31, 0x37, 0x35, 0x34,
        0x30, 0x37, 0x32, 0x2c, 0x22, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x69, 0x6c,
        0x6c, 0x69, 0x73, 0x22, 0x3a, 0x33, 0x34, 0x37, 0x39, 0x2c, 0x22, 0x70, 0x61, 0x75, 0x73, 0x65,
        0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a,
        0x30, 0x2c, 0x22, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x22, 0x3a,
        0x5b, 0x22, 0x34, 0x32, 0x22, 0x5d, 0x7d, 0x5d, 0x7d, 0x2c, 0x7b, 0x22, 0x5f, 0x6c, 0x69, 0x6e,
        0x6b, 0x73, 0x22, 0x3a, 0x7b, 0x22, 0x73, 0x65, 0x6c, 0x66, 0x22, 0x3a, 0x7b, 0x22, 0x68, 0x72,
        0x65, 0x66, 0x22, 0x3a, 0x22, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x47, 0x50, 0x4f, 0x4e, 0x2f, 0x6a,
        0x6f, 0x62, 0x2f, 0x4c, 0x54, 0x50, 0x2d, 0x58, 0x2d, 0x67, 0x69, 0x74, 0x2f, 0x6a, 0x6f, 0x62,
        0x2f, 0x4c, 0x54, 0x50, 0x2d, 0x38, 0x58, 0x2d, 0x52, 0x45, 0x56, 0x43, 0x2f, 0x6a, 0x6f, 0x62,
        0x2f, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2f, 0x32, 0x36, 0x36, 0x2f, 0x65, 0x78, 0x65, 0x63,
        0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x6e, 0x6f, 0x64, 0x65, 0x2f, 0x34, 0x37, 0x2f, 0x77, 0x66,
        0x61, 0x70, 0x69, 0x2f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x22, 0x7d, 0x7d, 0x2c,
        0x22, 0x69, 0x64, 0x22, 0x3a, 0x22, 0x34, 0x37, 0x22, 0x2c, 0x22, 0x6e, 0x61, 0x6d, 0x65, 0x22,
        0x3a, 0x22, 0x43, 0x6f, 0x64, 0x65, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x22, 0x2c, 0x22, 0x65,
        0x78, 0x65, 0x63, 0x4e, 0x6f, 0x64, 0x65, 0x22, 0x3a, 0x22, 0x22, 0x2c, 0x22, 0x73, 0x74, 0x61,
        0x74, 0x75, 0x73, 0x22, 0x3a, 0x22, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x22, 0x2c, 0x22,
        0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22,
        0x3a, 0x31, 0x35, 0x32, 0x36, 0x36, 0x33, 0x31, 0x37, 0x35, 0x37, 0x35, 0x36, 0x31, 0x2c, 0x22,
        0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a,
        0x39, 0x2c, 0x22, 0x70, 0x61, 0x75, 0x73, 0x65, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
        0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a, 0x30, 0x2c, 0x22, 0x73, 0x74, 0x61, 0x67, 0x65,
        0x46, 0x6c, 0x6f, 0x77, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x22, 0x3a, 0x5b, 0x7b, 0x22, 0x5f, 0x6c,
        0x69, 0x6e, 0x6b, 0x73, 0x22, 0x3a, 0x7b, 0x22, 0x73, 0x65, 0x6c, 0x66, 0x22, 0x3a, 0x7b, 0x22,
        0x68, 0x72, 0x65, 0x66, 0x22, 0x3a, 0x22, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x47, 0x50, 0x4f, 0x4e,
        0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x4c, 0x54, 0x50, 0x2d, 0x58, 0x2d, 0x67, 0x69, 0x74, 0x2f, 0x6a,
        0x6f, 0x62, 0x2f, 0x4c, 0x54, 0x50, 0x2d, 0x38, 0x58, 0x2d, 0x52, 0x45, 0x56, 0x43, 0x2f, 0x6a,
        0x6f, 0x62, 0x2f, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2f, 0x32, 0x36, 0x36, 0x2f, 0x65, 0x78,
        0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x6e, 0x6f, 0x64, 0x65, 0x2f, 0x34, 0x38, 0x2f,
        0x77, 0x66, 0x61, 0x70, 0x69, 0x2f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x22, 0x7d,
        0x2c, 0x22, 0x6c, 0x6f, 0x67, 0x22, 0x3a, 0x7b, 0x22, 0x68, 0x72, 0x65, 0x66, 0x22, 0x3a, 0x22,
        0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x47, 0x50, 0x4f, 0x4e, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x4c, 0x54,
        0x50, 0x2d, 0x58, 0x2d, 0x67, 0x69, 0x74, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x4c, 0x54, 0x50, 0x2d,
        0x38, 0x58, 0x2d, 0x52, 0x45, 0x56, 0x43, 0x2f, 0x6a, 0x6f, 0x62, 0x2f, 0x6d, 0x61, 0x73, 0x74,
        0x65, 0x72, 0x2f, 0x32, 0x36, 0x36, 0x2f, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6f, 0x6e,
        0x2f, 0x6e, 0x6f, 0x64, 0x65, 0x2f, 0x34, 0x38, 0x2f, 0x77, 0x66, 0x61, 0x70, 0x69, 0x2f, 0x6c,
        0x6f, 0x67, 0x22, 0x7d, 0x7d, 0x2c, 0x22, 0x69, 0x64, 0x22, 0x3a, 0x22, 0x34, 0x38, 0x22, 0x2c,
        0x22, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x3a, 0x22, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x20, 0x4d, 0x65,
        0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x2c, 0x22, 0x65, 0x78, 0x65, 0x63, 0x4e, 0x6f, 0x64, 0x65,
        0x22, 0x3a, 0x22, 0x22, 0x2c, 0x22, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x3a, 0x22, 0x53,
        0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x22, 0x2c, 0x22, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69,
        0x6d, 0x65, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a, 0x31, 0x35, 0x32, 0x36, 0x36, 0x33,
        0x31, 0x37, 0x35, 0x37, 0x35, 0x36, 0x36, 0x2c, 0x22, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
        0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22, 0x3a, 0x31, 0x2c, 0x22, 0x70, 0x61, 0x75, 0x73,
        0x65, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x22,
        0x3a, 0x30, 0x2c, 0x22, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x4e, 0x6f, 0x64, 0x65, 0x73, 0x22,
        0x3a, 0x5b, 0x22, 0x34, 0x37, 0x22, 0x5d, 0x7d, 0x5d, 0x7d, 0x5d, 0x7d, 0x5d];

        let res = parse_packet(&test_buf).unwrap();
        assert_eq!(res, Packet::IpV4 (ip::IpV4Packet {
            eth_header: ethernet::EthHeader {
                dst_mac: &[0xd0, 0x17, 0xc2, 0x97, 0x48, 0xb2],
                src_mac: &[0xa8, 0xf9, 0x4b, 0x80, 0xe7, 0x00],
            },
            vlans: None,
            ip_hdr: ip::IpV4Header{
                version: 4,
                hdr_length: 5,
                dscp: 0,
                ecn: 0,
                packet_length: 1471,
                id: 0xe546,
                flags: 0x2,
                offset: 0,
                ttl: 62,
                protocol: ip::IpNextHeaderProtocols::Tcp,
                crc: 0xce9f,
                src_ip: &[0xac as u8, 0x10,0x00,0x53],
                dst_ip: &[0xc0 as u8, 0xa8,0x16,0x47],
                options: None
            }
        }));
    }
}