use std::net::Ipv4Addr;
use std::str::FromStr;

use crate::ipman::ip::ip_from_bin;

use super::ip::ip_to_bin;
use super::mac::MacAddr;

pub(crate) fn convert_mac(bin: Option<String>, mac: Option<String>) {
    if bin.is_some() && mac.is_some() {
        eprintln!("only pass mac or bin!");
        std::process::exit(1);
    }

    if bin.is_none() && mac.is_none() {
        eprintln!("no arguments received!");
        std::process::exit(1);
    }

    if let Some(m) = mac {
        println!("{}", MacAddr::from_str(m).unwrap().to_bin());
    } else if let Some(b) = bin {
        println!("{}", MacAddr::from_bin(&b).unwrap().to_str());
    }
}

pub(crate) fn convert_ip(bin: Option<String>, ip: Option<String>) {
    if bin.is_some() && ip.is_some() {
        eprintln!("only pass ip or bin!");
        std::process::exit(1);
    }

    if bin.is_none() && ip.is_none() {
        eprintln!("no arguments received!");
        std::process::exit(1);
    }

    if let Some(i) = ip {
        println!("{}", ip_to_bin(Ipv4Addr::from_str(&i).unwrap()));
    } else if let Some(b) = bin {
        println!("{}", ip_from_bin(&b).unwrap());
    }
}
