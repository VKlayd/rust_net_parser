extern crate common;
extern crate ethertype;

pub fn parse_vlans <'a> (buf: &'a [u8]) -> Result<(Vec<u16>, usize), ()> {
    println!("{:?}", buf);
    let mut result : Vec<u16> = vec!();
    let mut ptr = 0;
    loop {
        let ethtype = ethertype::parse_ethtype(&buf[ptr..ptr+2]);
        ptr = ptr+2;
        if ethtype == ethertype::EtherTypes::Vlan {
            let vlan = common::u16_parse(&buf[ptr..ptr + 2]);
            ptr = ptr+2;
            result.push(vlan);
        } else {
            return Ok((result, ptr-2));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
