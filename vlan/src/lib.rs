extern crate common;
extern crate ethertype;

/// Represents an IEEE 802.1p class of service
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassOfService(pub u8);

impl ClassOfService {
    /// Create a new ClassOfService
    pub fn new(value: u8) -> ClassOfService {
        ClassOfService(value)
    }
}

/// IEEE 802.1p classes of service as defined in
/// https://en.wikipedia.org/wiki/IEEE_P802.1p
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ClassesOfService {
    use super::ClassOfService;

    /// Background
    pub const BK: ClassOfService = ClassOfService(1);

    /// Best Effort
    pub const BE: ClassOfService = ClassOfService(0);

    /// Excellent Effort
    pub const EE: ClassOfService = ClassOfService(2);

    /// Critical Applications
    pub const CA: ClassOfService = ClassOfService(3);

    /// Video, < 100 ms latency
    pub const VI: ClassOfService = ClassOfService(4);

    /// Voice, < 10 ms latency
    pub const VO: ClassOfService = ClassOfService(5);

    /// Internetwork Control
    pub const IC: ClassOfService = ClassOfService(6);

    /// Network Control
    pub const NC: ClassOfService = ClassOfService(7);
}

#[derive(Debug, PartialEq)]
pub struct Vlan {
    pub priority_code_point: ClassOfService,
    pub drop_eligible_indicator: bool,
    pub vid: u16,
}

fn parse_vlan(buf: &[u8]) -> Vlan {
    let vid = common::u16_parse(buf);

    let prio = (vid >> 13) as u8;
    let drop = (vid >> 12) as u8 & 1;

    Vlan{priority_code_point:ClassOfService(prio), drop_eligible_indicator: drop!=0, vid: vid & !(((prio as u16) << 13) | ((drop as u16) << 12))}
}

pub fn parse_vlans <'a> (buf: &'a [u8]) -> Result<(Option<Vec<Vlan>>, usize), ()> {
    let mut result : Vec<Vlan> = vec!();
    let mut ptr = 0;
    while ethertype::parse_ethtype(&buf[ptr..ptr+2]) == ethertype::EtherTypes::Vlan {
        ptr += 2;
        let vlan = parse_vlan(&buf[ptr..ptr+2]);
        ptr += 2;
        result.push(vlan);
    }
    match result.len() {
        x if x>0 => Ok((Some(result), ptr)),
        _ => Ok((None, ptr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_simplest_vlan() {
        let buf = [0 as u8, 1];
        let v = parse_vlan(&buf);
        assert!(v == Vlan{drop_eligible_indicator:false, priority_code_point:ClassesOfService::BE, vid:1})
    }
    #[test]
    fn check_vlan() {
        let buf = [0xC0 as u8, 1];
        let v = parse_vlan(&buf);
        assert!(v == Vlan{drop_eligible_indicator:false, priority_code_point:ClassesOfService::IC, vid:1})
    }
    #[test]
    fn check_full_vlan() {
        let buf = [0xBD as u8, 0x6B];
        let v = parse_vlan(&buf);
        assert!(v == Vlan{drop_eligible_indicator:true, priority_code_point:ClassOfService(5), vid:3435})
    }
}
