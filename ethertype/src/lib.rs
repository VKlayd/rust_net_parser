extern crate common;

use std::fmt;

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EtherTypes {
    use ::EtherType;
    /// Internet Protocol version 4 (IPv4) [RFC7042]
    pub const Ipv4: EtherType = EtherType(0x0800);
    /// Address Resolution Protocol (ARP) [RFC7042]
    pub const Arp: EtherType = EtherType(0x0806);
    /// Wake on Lan
    pub const WakeOnLan: EtherType = EtherType(0x0842);
    /// IETF TRILL Protocol [IEEE]
    pub const Trill: EtherType = EtherType(0x22F3);
    /// DECnet Phase IV
    pub const DECnet: EtherType = EtherType(0x6003);
    /// Reverse Address Resolution Protocol (RARP) [RFC903]
    pub const Rarp: EtherType = EtherType(0x8035);
    /// AppleTalk - EtherTalk [Apple]
    pub const AppleTalk: EtherType = EtherType(0x809B);
    /// AppleTalk Address Resolution Protocol (AARP) [Apple]
    pub const Aarp: EtherType = EtherType(0x80F3);
    /// IPX [Xerox]
    pub const Ipx: EtherType = EtherType(0x8137);
    /// QNX Qnet [QNX Software Systems]
    pub const Qnx: EtherType = EtherType(0x8204);
    /// Internet Protocol version 6 (IPv6) [RFC7042]
    pub const Ipv6: EtherType = EtherType(0x86DD);
    /// Ethernet Flow Control [IEEE 802.3x]
    pub const FlowControl: EtherType = EtherType(0x8808);
    /// CobraNet [CobraNet]
    pub const CobraNet: EtherType = EtherType(0x8819);
    /// MPLS Unicast [RFC 3032]
    pub const Mpls: EtherType = EtherType(0x8847);
    /// MPLS Multicast [RFC 5332]
    pub const MplsMcast: EtherType = EtherType(0x8848);
    /// PPPOE Discovery Stage [RFC 2516]
    pub const PppoeDiscovery: EtherType = EtherType(0x8863);
    /// PPPoE Session Stage [RFC 2516]
    pub const PppoeSession: EtherType = EtherType(0x8864);
    /// VLAN-tagged frame (IEEE 802.1Q)
    pub const Vlan: EtherType = EtherType(0x8100);
    /// Provider Bridging [IEEE 802.1ad / IEEE 802.1aq]
    pub const PBridge: EtherType = EtherType(0x88a8);
    /// Link Layer Discovery Protocol (LLDP) [IEEE 802.1AB]
    pub const Lldp: EtherType = EtherType(0x88cc);
    /// Precision Time Protocol (PTP) over Ethernet [IEEE 1588]
    pub const Ptp: EtherType = EtherType(0x88f7);
    /// CFM / Y.1731 [IEEE 802.1ag]
    pub const Cfm: EtherType = EtherType(0x8902);
    /// Q-in-Q Vlan Tagging [IEEE 802.1Q]
    pub const QinQ: EtherType = EtherType(0x9100);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EtherType(pub u16);

impl fmt::Display for EtherType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match self {
                   &EtherTypes::Ipv4 => "Ipv4", //(0x0800)
                   &EtherTypes::Arp => "Arp", //(0x0806)
                   &EtherTypes::WakeOnLan => "WakeOnLan", //(0x0842)
                   &EtherTypes::Trill => "Trill", //(0x22F3)
                   &EtherTypes::DECnet => "DECnet", //(0x6003)
                   &EtherTypes::Rarp => "Rarp", //(0x8035)
                   &EtherTypes::AppleTalk => "AppleTalk", //(0x809B)
                   &EtherTypes::Aarp => "Aarp", //(0x80F3)
                   &EtherTypes::Ipx => "Ipx", //(0x8137)
                   &EtherTypes::Qnx => "Qnx", //(0x8204)
                   &EtherTypes::Ipv6 => "Ipv6", //(0x86DD)
                   &EtherTypes::FlowControl => "FlowControl", //(0x8808)
                   &EtherTypes::CobraNet => "CobraNet", //(0x8819)
                   &EtherTypes::Mpls => "Mpls", //(0x8847)
                   &EtherTypes::MplsMcast => "MplsMcast", //(0x8848)
                   &EtherTypes::PppoeDiscovery => "PppoeDiscovery", //(0x8863)
                   &EtherTypes::PppoeSession => "PppoeSession", //(0x8864)
                   &EtherTypes::Vlan => "Vlan", //(0x8100)
                   &EtherTypes::PBridge => "PBridge", //(0x88a8)
                   &EtherTypes::Lldp => "Lldp", //(0x88cc)
                   &EtherTypes::Ptp => "Ptp", //(0x88f7)
                   &EtherTypes::Cfm => "Cfm", //(0x8902)
                   &EtherTypes::QinQ => "QinQ", //(0x9100)
                   _ => "unknown",
               })
    }
}

impl Into<u16> for EtherType {
    fn into(self) -> u16 {
        self.0
    }
}

impl From<u16> for EtherType {
    fn from(d: u16) -> Self {
        EtherType(d)
    }
}

pub fn parse_ethtype <'a> (buf: &'a [u8]) -> EtherType {
    EtherType(common::u16_parse(buf))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
