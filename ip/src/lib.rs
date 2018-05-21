#![feature(try_from)]
use std::convert::TryFrom;

extern crate common;
extern crate ethernet;
extern crate vlan;
extern crate ethertype;


#[allow(non_snake_case)]
pub mod Flags {
    /// Don't Fragment flag
    pub const DONT_FRAGMENT: u8 = 0b010;
    /// More Fragments flag
    pub const MORE_FRAGMENTS: u8 = 0b001;
}

/// IPv4 header options numbers as defined in
/// http://www.iana.org/assignments/ip-parameters/ip-parameters.xhtml
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Ipv4OptionNumbers {
    use super::Ipv4OptionNumber;

    /// End of Options List
    pub const EOL: Ipv4OptionNumber = Ipv4OptionNumber(0);

    /// No Operation
    pub const NOP: Ipv4OptionNumber = Ipv4OptionNumber(1);

    /// Security
    pub const SEC: Ipv4OptionNumber = Ipv4OptionNumber(2);

    /// Loose Source Route
    pub const LSR: Ipv4OptionNumber = Ipv4OptionNumber(3);

    /// Time Stamp
    pub const TS: Ipv4OptionNumber = Ipv4OptionNumber(4);

    /// Extended Security
    pub const ESEC: Ipv4OptionNumber = Ipv4OptionNumber(5);

    /// Commercial Security
    pub const CIPSO: Ipv4OptionNumber = Ipv4OptionNumber(6);

    /// Record Route
    pub const RR: Ipv4OptionNumber = Ipv4OptionNumber(7);

    /// Stream ID
    pub const SID: Ipv4OptionNumber = Ipv4OptionNumber(8);

    /// Strict Source Route
    pub const SSR: Ipv4OptionNumber = Ipv4OptionNumber(9);

    /// Experimental Measurement
    pub const ZSU: Ipv4OptionNumber = Ipv4OptionNumber(10);

    /// MTU Probe
    pub const MTUP: Ipv4OptionNumber = Ipv4OptionNumber(11);

    /// MTU Reply
    pub const MTUR: Ipv4OptionNumber = Ipv4OptionNumber(12);

    /// Experimental Flow Control
    pub const FINN: Ipv4OptionNumber = Ipv4OptionNumber(13);

    /// Experimental Access Control
    pub const VISA: Ipv4OptionNumber = Ipv4OptionNumber(14);

    /// ENCODE
    pub const ENCODE: Ipv4OptionNumber = Ipv4OptionNumber(15);

    /// IMI Traffic Descriptor
    pub const IMITD: Ipv4OptionNumber = Ipv4OptionNumber(16);

    /// Extended Internet Protocol
    pub const EIP: Ipv4OptionNumber = Ipv4OptionNumber(17);

    /// Traceroute
    pub const TR: Ipv4OptionNumber = Ipv4OptionNumber(18);

    /// Address Extension
    pub const ADDEXT: Ipv4OptionNumber = Ipv4OptionNumber(19);

    /// Router Alert
    pub const RTRALT: Ipv4OptionNumber = Ipv4OptionNumber(20);

    /// Selective Directed Broadcast
    pub const SDB: Ipv4OptionNumber = Ipv4OptionNumber(21);

    /// Dynamic Packet State
    pub const DPS: Ipv4OptionNumber = Ipv4OptionNumber(23);

    /// Upstream Multicast Pkt.
    pub const UMP: Ipv4OptionNumber = Ipv4OptionNumber(24);

    /// Quick-Start
    pub const QS: Ipv4OptionNumber = Ipv4OptionNumber(25);

    /// RFC3692-style Experiment
    pub const EXP: Ipv4OptionNumber = Ipv4OptionNumber(30);
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod IpNextHeaderProtocols {
    use super::IpNextHeaderProtocol;

    /// IPv6 Hop-by-Hop Option [RFC2460]
    pub const Hopopt: IpNextHeaderProtocol = IpNextHeaderProtocol(0);

    /// Internet Control Message [RFC792]
    pub const Icmp: IpNextHeaderProtocol = IpNextHeaderProtocol(1);

    /// Internet Group Management [RFC1112]
    pub const Igmp: IpNextHeaderProtocol = IpNextHeaderProtocol(2);

    /// Gateway-to-Gateway [RFC823]
    pub const Ggp: IpNextHeaderProtocol = IpNextHeaderProtocol(3);

    /// IPv4 encapsulation [RFC2003]
    pub const Ipv4: IpNextHeaderProtocol = IpNextHeaderProtocol(4);

    /// Stream [RFC1190][RFC1819]
    pub const St: IpNextHeaderProtocol = IpNextHeaderProtocol(5);

    /// Transmission Control [RFC793]
    pub const Tcp: IpNextHeaderProtocol = IpNextHeaderProtocol(6);

    /// CBT
    pub const Cbt: IpNextHeaderProtocol = IpNextHeaderProtocol(7);

    /// Exterior Gateway Protocol [RFC888]
    pub const Egp: IpNextHeaderProtocol = IpNextHeaderProtocol(8);

    /// any private interior gateway (used by Cisco for their IGRP)
    pub const Igp: IpNextHeaderProtocol = IpNextHeaderProtocol(9);

    /// BBN RCC Monitoring
    pub const BbnRccMon: IpNextHeaderProtocol = IpNextHeaderProtocol(10);

    /// Network Voice Protocol [RFC741]
    pub const NvpII: IpNextHeaderProtocol = IpNextHeaderProtocol(11);

    /// PUP
    pub const Pup: IpNextHeaderProtocol = IpNextHeaderProtocol(12);

    /// ARGUS
    pub const Argus: IpNextHeaderProtocol = IpNextHeaderProtocol(13);

    /// EMCON
    pub const Emcon: IpNextHeaderProtocol = IpNextHeaderProtocol(14);

    /// Cross Net Debugger
    pub const Xnet: IpNextHeaderProtocol = IpNextHeaderProtocol(15);

    /// Chaos
    pub const Chaos: IpNextHeaderProtocol = IpNextHeaderProtocol(16);

    /// User Datagram [RFC768]
    pub const Udp: IpNextHeaderProtocol = IpNextHeaderProtocol(17);

    /// Multiplexing
    pub const Mux: IpNextHeaderProtocol = IpNextHeaderProtocol(18);

    /// DCN Measurement Subsystems
    pub const DcnMeas: IpNextHeaderProtocol = IpNextHeaderProtocol(19);

    /// Host Monitoring [RFC869]
    pub const Hmp: IpNextHeaderProtocol = IpNextHeaderProtocol(20);

    /// Packet Radio Measurement
    pub const Prm: IpNextHeaderProtocol = IpNextHeaderProtocol(21);

    /// XEROX NS IDP
    pub const XnsIdp: IpNextHeaderProtocol = IpNextHeaderProtocol(22);

    /// Trunk-1
    pub const Trunk1: IpNextHeaderProtocol = IpNextHeaderProtocol(23);

    /// Trunk-2
    pub const Trunk2: IpNextHeaderProtocol = IpNextHeaderProtocol(24);

    /// Leaf-1
    pub const Leaf1: IpNextHeaderProtocol = IpNextHeaderProtocol(25);

    /// Leaf-2
    pub const Leaf2: IpNextHeaderProtocol = IpNextHeaderProtocol(26);

    /// Reliable Data Protocol [RFC908]
    pub const Rdp: IpNextHeaderProtocol = IpNextHeaderProtocol(27);

    /// Internet Reliable Transaction [RFC938]
    pub const Irtp: IpNextHeaderProtocol = IpNextHeaderProtocol(28);

    /// ISO Transport Protocol Class 4 [RFC905]
    pub const IsoTp4: IpNextHeaderProtocol = IpNextHeaderProtocol(29);

    /// Bulk Data Transfer Protocol [RFC969]
    pub const Netblt: IpNextHeaderProtocol = IpNextHeaderProtocol(30);

    /// MFE Network Services Protocol
    pub const MfeNsp: IpNextHeaderProtocol = IpNextHeaderProtocol(31);

    /// MERIT Internodal Protocol
    pub const MeritInp: IpNextHeaderProtocol = IpNextHeaderProtocol(32);

    /// Datagram Congestion Control Protocol [RFC4340]
    pub const Dccp: IpNextHeaderProtocol = IpNextHeaderProtocol(33);

    /// Third Party Connect Protocol
    pub const ThreePc: IpNextHeaderProtocol = IpNextHeaderProtocol(34);

    /// Inter-Domain Policy Routing Protocol
    pub const Idpr: IpNextHeaderProtocol = IpNextHeaderProtocol(35);

    /// XTP
    pub const Xtp: IpNextHeaderProtocol = IpNextHeaderProtocol(36);

    /// Datagram Delivery Protocol
    pub const Ddp: IpNextHeaderProtocol = IpNextHeaderProtocol(37);

    /// IDPR Control Message Transport Proto
    pub const IdprCmtp: IpNextHeaderProtocol = IpNextHeaderProtocol(38);

    /// TP++ Transport Protocol
    pub const TpPlusPlus: IpNextHeaderProtocol = IpNextHeaderProtocol(39);

    /// IL Transport Protocol
    pub const Il: IpNextHeaderProtocol = IpNextHeaderProtocol(40);

    /// IPv6 encapsulation [RFC2473]
    pub const Ipv6: IpNextHeaderProtocol = IpNextHeaderProtocol(41);

    /// Source Demand Routing Protocol
    pub const Sdrp: IpNextHeaderProtocol = IpNextHeaderProtocol(42);

    /// Routing Header for IPv6
    pub const Ipv6Route: IpNextHeaderProtocol = IpNextHeaderProtocol(43);

    /// Fragment Header for IPv6
    pub const Ipv6Frag: IpNextHeaderProtocol = IpNextHeaderProtocol(44);

    /// Inter-Domain Routing Protocol
    pub const Idrp: IpNextHeaderProtocol = IpNextHeaderProtocol(45);

    /// Reservation Protocol [RFC2205][RFC3209]
    pub const Rsvp: IpNextHeaderProtocol = IpNextHeaderProtocol(46);

    /// Generic Routing Encapsulation [RFC1701]
    pub const Gre: IpNextHeaderProtocol = IpNextHeaderProtocol(47);

    /// Dynamic Source Routing Protocol [RFC4728]
    pub const Dsr: IpNextHeaderProtocol = IpNextHeaderProtocol(48);

    /// BNA
    pub const Bna: IpNextHeaderProtocol = IpNextHeaderProtocol(49);

    /// Encap Security Payload [RFC4303]
    pub const Esp: IpNextHeaderProtocol = IpNextHeaderProtocol(50);

    /// Authentication Header [RFC4302]
    pub const Ah: IpNextHeaderProtocol = IpNextHeaderProtocol(51);

    /// Integrated Net Layer Security TUBA
    pub const INlsp: IpNextHeaderProtocol = IpNextHeaderProtocol(52);

    /// IP with Encryption
    pub const Swipe: IpNextHeaderProtocol = IpNextHeaderProtocol(53);

    /// NBMA Address Resolution Protocol [RFC1735]
    pub const Narp: IpNextHeaderProtocol = IpNextHeaderProtocol(54);

    /// IP Mobility
    pub const Mobile: IpNextHeaderProtocol = IpNextHeaderProtocol(55);

    /// Transport Layer Security Protocol using Kryptonet key management
    pub const Tlsp: IpNextHeaderProtocol = IpNextHeaderProtocol(56);

    /// SKIP
    pub const Skip: IpNextHeaderProtocol = IpNextHeaderProtocol(57);

    #[deprecated(note="Please use `IpNextHeaderProtocols::Icmpv6` instead")]
    pub const Ipv6Icmp: IpNextHeaderProtocol = IpNextHeaderProtocol(58);

    /// ICMPv6 [RFC4443]
    pub const Icmpv6: IpNextHeaderProtocol = IpNextHeaderProtocol(58);

    /// No Next Header for IPv6 [RFC2460]
    pub const Ipv6NoNxt: IpNextHeaderProtocol = IpNextHeaderProtocol(59);

    /// Destination Options for IPv6 [RFC2460]
    pub const Ipv6Opts: IpNextHeaderProtocol = IpNextHeaderProtocol(60);

    /// any host internal protocol
    pub const HostInternal: IpNextHeaderProtocol = IpNextHeaderProtocol(61);

    /// CFTP
    pub const Cftp: IpNextHeaderProtocol = IpNextHeaderProtocol(62);

    /// any local network
    pub const LocalNetwork: IpNextHeaderProtocol = IpNextHeaderProtocol(63);

    /// SATNET and Backroom EXPAK
    pub const SatExpak: IpNextHeaderProtocol = IpNextHeaderProtocol(64);

    /// Kryptolan
    pub const Kryptolan: IpNextHeaderProtocol = IpNextHeaderProtocol(65);

    /// MIT Remote Virtual Disk Protocol
    pub const Rvd: IpNextHeaderProtocol = IpNextHeaderProtocol(66);

    /// Internet Pluribus Packet Core
    pub const Ippc: IpNextHeaderProtocol = IpNextHeaderProtocol(67);

    /// any distributed file system
    pub const DistributedFs: IpNextHeaderProtocol = IpNextHeaderProtocol(68);

    /// SATNET Monitoring
    pub const SatMon: IpNextHeaderProtocol = IpNextHeaderProtocol(69);

    /// VISA Protocol
    pub const Visa: IpNextHeaderProtocol = IpNextHeaderProtocol(70);

    /// Internet Packet Core Utility
    pub const Ipcv: IpNextHeaderProtocol = IpNextHeaderProtocol(71);

    /// Computer Protocol Network Executive
    pub const Cpnx: IpNextHeaderProtocol = IpNextHeaderProtocol(72);

    /// Computer Protocol Heart Beat
    pub const Cphb: IpNextHeaderProtocol = IpNextHeaderProtocol(73);

    /// Wang Span Network
    pub const Wsn: IpNextHeaderProtocol = IpNextHeaderProtocol(74);

    /// Packet Video Protocol
    pub const Pvp: IpNextHeaderProtocol = IpNextHeaderProtocol(75);

    /// Backroom SATNET Monitoring
    pub const BrSatMon: IpNextHeaderProtocol = IpNextHeaderProtocol(76);

    /// SUN ND PROTOCOL-Temporary
    pub const SunNd: IpNextHeaderProtocol = IpNextHeaderProtocol(77);

    /// WIDEBAND Monitoring
    pub const WbMon: IpNextHeaderProtocol = IpNextHeaderProtocol(78);

    /// WIDEBAND EXPAK
    pub const WbExpak: IpNextHeaderProtocol = IpNextHeaderProtocol(79);

    /// ISO Internet Protocol
    pub const IsoIp: IpNextHeaderProtocol = IpNextHeaderProtocol(80);

    /// VMTP
    pub const Vmtp: IpNextHeaderProtocol = IpNextHeaderProtocol(81);

    /// SECURE-VMTP
    pub const SecureVmtp: IpNextHeaderProtocol = IpNextHeaderProtocol(82);

    /// VINES
    pub const Vines: IpNextHeaderProtocol = IpNextHeaderProtocol(83);

    /// Transaction Transport Protocol/IP Traffic Manager
    pub const TtpOrIptm: IpNextHeaderProtocol = IpNextHeaderProtocol(84);

    /// NSFNET-IGP
    pub const NsfnetIgp: IpNextHeaderProtocol = IpNextHeaderProtocol(85);

    /// Dissimilar Gateway Protocol
    pub const Dgp: IpNextHeaderProtocol = IpNextHeaderProtocol(86);

    /// TCF
    pub const Tcf: IpNextHeaderProtocol = IpNextHeaderProtocol(87);

    /// EIGRP
    pub const Eigrp: IpNextHeaderProtocol = IpNextHeaderProtocol(88);

    /// OSPFIGP [RFC1583][RFC2328][RFC5340]
    pub const OspfigP: IpNextHeaderProtocol = IpNextHeaderProtocol(89);

    /// Sprite RPC Protocol
    pub const SpriteRpc: IpNextHeaderProtocol = IpNextHeaderProtocol(90);

    /// Locus Address Resolution Protocol
    pub const Larp: IpNextHeaderProtocol = IpNextHeaderProtocol(91);

    /// Multicast Transport Protocol
    pub const Mtp: IpNextHeaderProtocol = IpNextHeaderProtocol(92);

    /// AX.25 Frames
    pub const Ax25: IpNextHeaderProtocol = IpNextHeaderProtocol(93);

    /// IP-within-IP Encapsulation Protocol
    pub const IpIp: IpNextHeaderProtocol = IpNextHeaderProtocol(94);

    /// Mobile Internetworking Control Pro.
    pub const Micp: IpNextHeaderProtocol = IpNextHeaderProtocol(95);

    /// Semaphore Communications Sec. Pro.
    pub const SccSp: IpNextHeaderProtocol = IpNextHeaderProtocol(96);

    /// Ethernet-within-IP Encapsulation [RFC3378]
    pub const Etherip: IpNextHeaderProtocol = IpNextHeaderProtocol(97);

    /// Encapsulation Header [RFC1241]
    pub const Encap: IpNextHeaderProtocol = IpNextHeaderProtocol(98);

    /// any private encryption scheme
    pub const PrivEncryption: IpNextHeaderProtocol = IpNextHeaderProtocol(99);

    /// GMTP
    pub const Gmtp: IpNextHeaderProtocol = IpNextHeaderProtocol(100);

    /// Ipsilon Flow Management Protocol
    pub const Ifmp: IpNextHeaderProtocol = IpNextHeaderProtocol(101);

    /// PNNI over IP
    pub const Pnni: IpNextHeaderProtocol = IpNextHeaderProtocol(102);

    /// Protocol Independent Multicast [RFC4601]
    pub const Pim: IpNextHeaderProtocol = IpNextHeaderProtocol(103);

    /// ARIS
    pub const Aris: IpNextHeaderProtocol = IpNextHeaderProtocol(104);

    /// SCPS
    pub const Scps: IpNextHeaderProtocol = IpNextHeaderProtocol(105);

    /// QNX
    pub const Qnx: IpNextHeaderProtocol = IpNextHeaderProtocol(106);

    /// Active Networks
    pub const AN: IpNextHeaderProtocol = IpNextHeaderProtocol(107);

    /// IP Payload Compression Protocol [RFC2393]
    pub const IpComp: IpNextHeaderProtocol = IpNextHeaderProtocol(108);

    /// Sitara Networks Protocol
    pub const Snp: IpNextHeaderProtocol = IpNextHeaderProtocol(109);

    /// Compaq Peer Protocol
    pub const CompaqPeer: IpNextHeaderProtocol = IpNextHeaderProtocol(110);

    /// IPX in IP
    pub const IpxInIp: IpNextHeaderProtocol = IpNextHeaderProtocol(111);

    /// Virtual Router Redundancy Protocol [RFC5798]
    pub const Vrrp: IpNextHeaderProtocol = IpNextHeaderProtocol(112);

    /// PGM Reliable Transport Protocol
    pub const Pgm: IpNextHeaderProtocol = IpNextHeaderProtocol(113);

    /// any 0-hop protocol
    pub const ZeroHop: IpNextHeaderProtocol = IpNextHeaderProtocol(114);

    /// Layer Two Tunneling Protocol [RFC3931]
    pub const L2tp: IpNextHeaderProtocol = IpNextHeaderProtocol(115);

    /// D-II Data Exchange (DDX)
    pub const Ddx: IpNextHeaderProtocol = IpNextHeaderProtocol(116);

    /// Interactive Agent Transfer Protocol
    pub const Iatp: IpNextHeaderProtocol = IpNextHeaderProtocol(117);

    /// Schedule Transfer Protocol
    pub const Stp: IpNextHeaderProtocol = IpNextHeaderProtocol(118);

    /// SpectraLink Radio Protocol
    pub const Srp: IpNextHeaderProtocol = IpNextHeaderProtocol(119);

    /// UTI
    pub const Uti: IpNextHeaderProtocol = IpNextHeaderProtocol(120);

    /// Simple Message Protocol
    pub const Smp: IpNextHeaderProtocol = IpNextHeaderProtocol(121);

    /// Simple Multicast Protocol
    pub const Sm: IpNextHeaderProtocol = IpNextHeaderProtocol(122);

    /// Performance Transparency Protocol
    pub const Ptp: IpNextHeaderProtocol = IpNextHeaderProtocol(123);

    ///
    pub const IsisOverIpv4: IpNextHeaderProtocol = IpNextHeaderProtocol(124);

    ///
    pub const Fire: IpNextHeaderProtocol = IpNextHeaderProtocol(125);

    /// Combat Radio Transport Protocol
    pub const Crtp: IpNextHeaderProtocol = IpNextHeaderProtocol(126);

    /// Combat Radio User Datagram
    pub const Crudp: IpNextHeaderProtocol = IpNextHeaderProtocol(127);

    ///
    pub const Sscopmce: IpNextHeaderProtocol = IpNextHeaderProtocol(128);

    ///
    pub const Iplt: IpNextHeaderProtocol = IpNextHeaderProtocol(129);

    /// Secure Packet Shield
    pub const Sps: IpNextHeaderProtocol = IpNextHeaderProtocol(130);

    /// Private IP Encapsulation within IP
    pub const Pipe: IpNextHeaderProtocol = IpNextHeaderProtocol(131);

    /// Stream Control Transmission Protocol
    pub const Sctp: IpNextHeaderProtocol = IpNextHeaderProtocol(132);

    /// Fibre Channel [RFC6172]
    pub const Fc: IpNextHeaderProtocol = IpNextHeaderProtocol(133);

    /// [RFC3175]
    pub const RsvpE2eIgnore: IpNextHeaderProtocol = IpNextHeaderProtocol(134);

    /// [RFC6275]
    pub const MobilityHeader: IpNextHeaderProtocol = IpNextHeaderProtocol(135);

    /// [RFC3828]
    pub const UdpLite: IpNextHeaderProtocol = IpNextHeaderProtocol(136);

    /// [RFC4023]
    pub const MplsInIp: IpNextHeaderProtocol = IpNextHeaderProtocol(137);

    /// MANET Protocols [RFC5498]
    pub const Manet: IpNextHeaderProtocol = IpNextHeaderProtocol(138);

    /// Host Identity Protocol [RFC5201]
    pub const Hip: IpNextHeaderProtocol = IpNextHeaderProtocol(139);

    /// Shim6 Protocol [RFC5533]
    pub const Shim6: IpNextHeaderProtocol = IpNextHeaderProtocol(140);

    /// Wrapped Encapsulating Security Payload [RFC5840]
    pub const Wesp: IpNextHeaderProtocol = IpNextHeaderProtocol(141);

    /// Robust Header Compression [RFC5858]
    pub const Rohc: IpNextHeaderProtocol = IpNextHeaderProtocol(142);

    /// Use for experimentation and testing [RFC3692]
    pub const Test1: IpNextHeaderProtocol = IpNextHeaderProtocol(253);

    /// Use for experimentation and testing [RFC3692]
    pub const Test2: IpNextHeaderProtocol = IpNextHeaderProtocol(254);

    ///
    pub const Reserved: IpNextHeaderProtocol = IpNextHeaderProtocol(255);

}

/// Represents an IPv4 next level protocol, or an IPv6 next header protocol,
/// see `IpNextHeaderProtocols` for a list of values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IpNextHeaderProtocol(pub u8);

impl IpNextHeaderProtocol {
    /// Create a new IpNextHeaderProtocol
    pub fn new(value: u8) -> IpNextHeaderProtocol {
        IpNextHeaderProtocol(value)
    }
}

/// Represents an IPv4 option
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Ipv4OptionNumber(pub u8);

impl Ipv4OptionNumber {
    /// Create a new Ipv4OptionNumber
    pub fn new(value: u8) -> Ipv4OptionNumber { Ipv4OptionNumber(value) }
}

#[derive(Debug,PartialEq)]
pub struct IpOption <'a> {
    pub copy: bool,
    pub class: u8,
    pub option_num: Ipv4OptionNumber,
    pub option_size: u8,
    pub val: &'a[u8]
}

#[derive(Debug,PartialEq)]
pub struct IpV4Header <'a>{
    pub version: u8,
    pub hdr_length: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub packet_length: u16,
    pub id: u16,
    pub flags: u8,
    pub offset: u16,
    pub ttl: u8,
    pub protocol: IpNextHeaderProtocol,
    pub crc: u16,
    pub src_ip: &'a[u8],
    pub dst_ip: &'a[u8],
    pub options: Option<Vec<IpOption<'a>>>
}

#[derive(Debug,PartialEq)]
pub struct IpV4Packet <'a> {
    pub eth_header: ethernet::EthHeader<'a>,
    pub vlans : Option<Vec<vlan::Vlan>>,
    pub ip_hdr : IpV4Header<'a>,
}


fn parse_options<'a>(buf: &'a[u8]) -> Result<Option<Vec<IpOption<'a>>>, ()> {
    let len = buf.len();
    let check = len % 4;
    if check != 0 {
        return Err(());
    }

    let mut result: Vec<IpOption> = vec!();

    let mut offset = 0;
    while offset < len {
        let copy = buf[offset] >> 7;
        let class = buf[offset] >> 5 & !(1 << 2);
        let option_num = Ipv4OptionNumber(buf[offset] & !!(7 << 5));

        if option_num == Ipv4OptionNumbers::EOL {
            break;
        }

        offset += 1;
        let option_size = buf[offset];
        offset += 1;
        let val = &buf[offset..offset + option_size as usize];
        result.push(IpOption{copy: copy!=0, class, option_num, option_size, val});
        offset += option_size  as usize;
    }

    if result.len() > 0 {
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

impl <'a>TryFrom<&'a [u8]> for IpV4Header<'a> {
    type Error = ();

    fn try_from(buf: &'a [u8]) -> Result<IpV4Header, Self::Error> {
        if buf.len()>=20 {
            let version = buf[0] >> 4;
            let hdr_length = buf[0] & 0xF;
            if hdr_length < 5 || hdr_length > 15 {
                return Err(());
            } else {
                if hdr_length as usize * 4 > buf.len() {
                    return Err(());
                }
            }
            let dscp = buf[1] >> 2;
            let ecn = buf[1] & 0x3;
            let packet_length = common::u16_parse(&buf[2..=3]);
            let id = common::u16_parse(&buf[4..=5]);
            let flags = buf[6] >> 5;
            let offset = common::u16_parse(&buf[6..=7]) & !((flags as u16) << 13);
            let ttl = buf[8];
            let protocol = IpNextHeaderProtocol(buf[9]);
            let crc = common::u16_parse(&buf[10..=11]);

            Ok(IpV4Header {
                version,
                hdr_length,
                dscp,
                ecn,
                packet_length,
                id,
                flags,
                offset,
                ttl,
                protocol,
                crc,
                src_ip: &buf[12..=15],
                dst_ip: &buf[16..=19],
                options: match hdr_length > 5 {
                    true => match parse_options(&buf[20..hdr_length as usize * 4]) {
                        Ok(x) => x,
                        _ => {
                            return Err(())
                        }
                    }
                    _ => None
                },
            })
        } else {
            Err(())
        }
    }
}

impl <'a>TryFrom<&'a [u8]> for IpV4Packet<'a> {
    type Error = ();

    fn try_from(buf: &'a [u8]) -> Result<IpV4Packet, Self::Error> {
        let eth_end = 12;

        let (v, ptr) = vlan::parse_vlans(&buf[eth_end..buf.len()]).unwrap();

        let ethtype = ethertype::parse_ethtype(&buf[eth_end + ptr..eth_end + ptr + 2]);
        if ethtype == ethertype::EtherTypes::Ipv4 {
            Ok(IpV4Packet { eth_header: ethernet::EthHeader::try_from(buf)?, vlans: v, ip_hdr: IpV4Header::try_from(&buf[eth_end + ptr + 2..buf.len()])?})
        } else {
            Err(())
        }
    }
}

impl <'a> TryFrom<(ethernet::EthHeader<'a>, Option<Vec<vlan::Vlan>>, &'a [u8])> for IpV4Packet<'a> {
    type Error = ();
    fn try_from(src: (ethernet::EthHeader<'a>, Option<Vec<vlan::Vlan>>, &'a [u8])) -> Result<IpV4Packet, Self::Error> {
        let (eth_header, vlans, payload) = src;
        Ok(IpV4Packet{eth_header, vlans, ip_hdr: IpV4Header::try_from(&payload[..])?})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ok() {
        let buf = [0x45 as u8,0x00,0x05,0xbf,0xe5,0x46,0x40,0x00,0x3e,0x06,0xce,0x9f,0xac,0x10,0x00,0x53,0xc0,0xa8,0x16,0x47];
        let eth = IpV4Header::try_from(&buf[..]);
        assert!(eth == Ok(IpV4Header{
            version: 4,
            hdr_length: 5,
            dscp: 0,
            ecn: 0,
            packet_length: 1471,
            id: 0xe546,
            flags: 0x2,
            offset: 0,
            ttl: 62,
            protocol: IpNextHeaderProtocol(6),
            crc: 0xce9f,
            src_ip: &[0xac as u8, 0x10,0x00,0x53],
            dst_ip: &[0xc0 as u8, 0xa8,0x16,0x47],
            options: None
        }));
    }
    #[test]
    fn test_broken() {
        let buf = [0x45 as u8,0x00,0x05,0xbf,0xe5,0x46,0x40,0x00,0x3e,0x06,0xce,0x9f,0xac,0x10,0x00,0x53,0xc0,0xa8];
        let eth = IpV4Header::try_from(&buf[..]);
        assert!(eth == Err(()));
    }
    #[test]
    fn test_broken_hdr_len() {
        let buf = [0x46 as u8,0x00,0x05,0xbf,0xe5,0x46,0x40,0x00,0x3e,0x06,0xce,0x9f,0xac,0x10,0x00,0x53,0xc0,0xa8,0x16,0x47];
        let eth = IpV4Header::try_from(&buf[..]);
        assert!(eth == Err(()));
    }
    #[test]
    fn test_broken_options() {
        let buf = [0x46 as u8,0x00,0x05,0xbf,0xe5,0x46,0x40,0x00,0x3e,0x06,0xce,0x9f,0xac,0x10,0x00,0x53,0xc0,0xa8,0x16,0x47,0x22];
        let eth = IpV4Header::try_from(&buf[..]);
        assert!(eth == Err(()));
    }
}