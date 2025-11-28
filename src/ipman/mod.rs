pub(crate) mod cli;
mod consts;
mod convert;
mod ip;
mod mac;
mod table;

use clap::Parser;
use cli::{IpCli, IpCommands};
use convert::{convert_ip, convert_mac};
use table::{mk_classful_table, print_table};

pub fn run_cli() {
    let args = IpCli::parse();

    match args.command {
        IpCommands::ConvertIp { bin, ip } => convert_ip(bin, ip),
        IpCommands::ConvertMac { bin, mac } => convert_mac(bin, mac),
        IpCommands::Table {
            base_ip,
            num_subnets,
            num_hosts,
        } => print_table(mk_classful_table(base_ip, num_subnets, num_hosts)),
    };
}
