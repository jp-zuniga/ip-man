use std::net::Ipv4Addr;

#[derive(Debug)]
pub(crate) struct Subnet {
    pub(crate) id: Ipv4Addr,
    pub(crate) host_range: (Ipv4Addr, Ipv4Addr),
    pub(crate) broadcast: Ipv4Addr,
    pub(crate) mask: Ipv4Addr,
}

pub(crate) fn mk_classful_table(
    base: Ipv4Addr,
    subnets: u32,
    hosts: u32,
) -> Vec<Subnet> {
    let mut table: Vec<Subnet> = Vec::new();

    table.reserve_exact(subnets as usize);

    let required_size = hosts + 2;
    let block_size = required_size.next_power_of_two();
    let mask_bits = !(block_size - 1);

    let mask = Ipv4Addr::from(mask_bits);

    let mut current_net_bits = base.to_bits();

    for _ in 0..subnets {
        let net_id = Ipv4Addr::from(current_net_bits);

        let broadcast_bits = current_net_bits | (block_size - 1);

        let broadcast = Ipv4Addr::from(broadcast_bits);
        let first_ip = Ipv4Addr::from(current_net_bits + 1);
        let last_ip = Ipv4Addr::from(broadcast_bits - 1);

        table.push(Subnet {
            id: net_id,
            host_range: (first_ip, last_ip),
            broadcast,
            mask,
        });

        current_net_bits += block_size;
    }

    table
}

pub(crate) fn mk_vlsm_table(base: Ipv4Addr, mut needs: Vec<u32>) -> Vec<Subnet> {
    let mut table: Vec<Subnet> = Vec::new();

    table.reserve_exact(needs.len());
    needs.sort_unstable_by(|a, b| b.cmp(a));

    let mut current_net_bits = base.to_bits();

    for hosts_needed in needs {
        let required_size = hosts_needed + 2;
        let block_size = required_size.next_power_of_two();

        let mask_bits = !(block_size - 1);
        let broadcast_bits = current_net_bits | (block_size - 1);

        let mask = Ipv4Addr::from(mask_bits);
        let net_id = Ipv4Addr::from(current_net_bits);
        let broadcast = Ipv4Addr::from(broadcast_bits);

        let first_ip = Ipv4Addr::from(current_net_bits + 1);
        let last_ip = Ipv4Addr::from(broadcast_bits - 1);

        table.push(Subnet {
            id: net_id,
            host_range: (first_ip, last_ip),
            broadcast,
            mask,
        });

        current_net_bits += block_size;
    }

    table
}
