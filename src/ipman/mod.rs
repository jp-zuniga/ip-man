pub(crate) mod cli;
mod consts;
mod convert;
mod ip;
mod mac;
mod table;

use clap::Parser;
use cli::{IpCli, IpCommands};
use convert::{convert_ip, convert_mac};

pub fn run_cli() {
    let args = IpCli::parse();

    match args.command {
        IpCommands::ConvertIp { bin, ip } => convert_ip(bin, ip),
        IpCommands::ConvertMac { bin, mac } => convert_mac(bin, mac),
        _ => {}
    };
}
