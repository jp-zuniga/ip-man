use std::net::Ipv4Addr;
use std::str::FromStr;

use super::consts::BASE_2;

pub(crate) fn ip_to_bin(ip: Ipv4Addr) -> String {
    let mut bin_ip = String::new();

    let octets = ip.octets();

    for oct in octets {
        let join = if &oct != octets.iter().last().unwrap() {
            "."
        } else {
            ""
        };

        bin_ip.push_str(
            format!(
                "{}{:b}{}",
                "0".repeat(oct.leading_zeros() as usize),
                oct,
                join
            )
            .as_str(),
        );
    }

    bin_ip
}

pub(crate) fn ip_from_bin(bin_ip: &str) -> Result<Ipv4Addr, ()> {
    let split_octets: Vec<&str> = bin_ip.split(".").collect();

    let split_len = split_octets.len();

    let mut new_ip = String::new();

    for (idx, pair) in split_octets.iter().enumerate() {
        let conversion = u8::from_str_radix(pair, BASE_2);

        if conversion.is_err() {
            return Err(());
        }

        let join = if idx != split_len - 1 { "." } else { "" };

        new_ip.push_str(
            format!(
                "{}{join}",
                conversion.expect("err should have already been returned")
            )
            .to_string()
            .as_str(),
        );
    }

    Ok(Ipv4Addr::from_str(&new_ip).expect("octets in mapped array should be valid u8"))
}
