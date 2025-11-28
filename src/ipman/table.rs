use std::net::Ipv4Addr;

#[derive(Debug)]
pub(crate) struct Subnet {
    pub(crate) id: Ipv4Addr,
    pub(crate) host_range: (Ipv4Addr, Ipv4Addr),
    pub(crate) broadcast: Ipv4Addr,
    pub(crate) mask: Ipv4Addr,
}

pub(crate) fn mk_classful_table(base: Ipv4Addr, subnets: u32, hosts: u32) -> Vec<Subnet> {
    let mut table: Vec<Subnet> = Vec::new();

    table.reserve_exact(subnets as usize);

    let mut cur_octets = base.octets();

    for _ in 1..=subnets {
        let net_id = Ipv4Addr::from(cur_octets);

        incr_octets(&mut cur_octets);

        let first_ip = Ipv4Addr::from(cur_octets);
        let mut last_ip = Ipv4Addr::from(cur_octets);
        let mut broadcast = Ipv4Addr::from(cur_octets);

        for host in 1..=hosts {
            if host == hosts - 2 {
                last_ip = Ipv4Addr::from(cur_octets);
                incr_octets(&mut cur_octets);
                broadcast = Ipv4Addr::from(cur_octets);
            }

            incr_octets(&mut cur_octets);
        }

        table.push(Subnet {
            id: net_id,
            host_range: (first_ip, last_ip),
            broadcast,
            mask: broadcast,
        });
    }

    table
}

pub(crate) fn mk_vlsm_table(base: Ipv4Addr, subnets: u16, hosts: u16) -> Vec<Subnet> {
    let mut table: Vec<Subnet> = Vec::new();

    table.reserve_exact(subnets as usize);

    table
}

fn incr_octets(octs: &mut [u8; 4]) {
    if octs[3] == 255 {
        octs[3] = 0;
        octs[2] += 1;
    }

    if octs[2] == 255 {
        octs[2] = 0;
        octs[1] += 1;
    }

    if octs[1] == 255 {
        octs[1] = 0;
        octs[1] += 1;
    }

    octs[3] += 1;
}

pub(crate) fn print_table(table: Vec<Subnet>) {
    for subnet in table {
        println!("{:#?}", subnet);
    }
}
