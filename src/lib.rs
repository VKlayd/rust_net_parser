extern crate common;
extern crate ethernet;
extern crate ethertype;
extern crate vlan;
extern crate arp;

#[derive(Debug,PartialEq)]
enum Packet <'a>{
    Arp {ethernet: ethernet::EthHeader<'a>, vlans: Vec<u16>, arp_hdr: arp::ArpHdr, arp_body: arp::ArpBody<'a>},
    Unknown {ethernet: ethernet::EthHeader<'a>, vlans: Vec<u16>, ethertype: ethertype::EtherType}
}

fn parse_packet <'a> (buf: &'a [u8]) -> Result<Packet, ()> {
    let eth_end = 12;

    let (v, ptr) = vlan::parse_vlans(&buf[eth_end..buf.len()]).unwrap();

    let ethtype = ethertype::parse_ethtype(&buf[eth_end + ptr .. eth_end + ptr+2]);

    match ethtype {
        ethertype::EtherTypes::Arp => {
            let (hdr, body) = arp::parse_packet(&buf[eth_end + ptr + 2 .. buf.len()]).unwrap();
            Ok(Packet::Arp {ethernet: ethernet::eth_parse(buf), vlans:v, arp_hdr: hdr, arp_body: body})
        },
        _ => {
            Ok(Packet::Unknown {ethernet: ethernet::eth_parse(buf), vlans:v, ethertype: ethtype})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_arp() {
        let test_buf = [0x11 as u8,0x12,0x13,0x14,0x15,0x16,0x01,0x02,0x03,0x04,0x05,0x06,0x81,0x00,0x00,0x01,0x08,0x06,0x0,0x1,0x08,0,6,4,0,1,0x01,0x02,0x03,0x04,0x05,0x06,0x1,0x1,0x1,0x1,0,0,0,0,0,0,0x2,0x2,0x2,0x2];
        let res = parse_packet(&test_buf).unwrap();
        assert_eq!(res, Packet::Arp {
            ethernet: ethernet::EthHeader {
                dst_mac: &[17, 18, 19, 20, 21, 22], src_mac: &[1, 2, 3, 4, 5, 6] },
            vlans: vec!(1),
            arp_hdr: arp::ArpHdr {
                ar_hrd: 1, ar_pro: 2048, ar_hln: 6, ar_pln: 4, ar_op: 1 },
            arp_body: arp::ArpBody {
                source: arp::ArpInfo {
                    ar_ha: &[1, 2, 3, 4, 5, 6], addr: &[1, 1, 1, 1] },
                target: arp::ArpInfo {
                    ar_ha: &[0, 0, 0, 0, 0, 0], addr: &[2, 2, 2, 2] } } } );
    }
    #[test]
    fn check_not_arp() {
        let test_buf = [0x11 as u8,0x12,0x13,0x14,0x15,0x16,0x01,0x02,0x03,0x04,0x05,0x06,0x81,0x00,0x00,0x01,0x08,0x08,0x0,0x1,0x08,0,6,4,0,1,0x01,0x02,0x03,0x04,0x05,0x06,0x1,0x1,0x1,0x1,0,0,0,0,0,0,0x2,0x2,0x2,0x2];
        let res = parse_packet(&test_buf).unwrap();
        assert_eq!(res, Packet::Unknown {
            ethernet: ethernet::EthHeader {
                dst_mac: &[17, 18, 19, 20, 21, 22], src_mac: &[1, 2, 3, 4, 5, 6] },
            vlans: vec!(1), ethertype: ethertype::EtherType(0x0808)} );
    }
}