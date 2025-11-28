use std::net::Ipv4Addr;

use clap::{Parser, Subcommand};
use clap::{crate_authors, crate_version};

use super::consts::CLI_STYLE;

#[derive(Debug, Subcommand)]
pub(crate) enum IpCommands {
    ConvertIp {
        #[arg(short, long)]
        bin: Option<String>,

        #[arg(short, long)]
        ip: Option<String>,
    },
    ConvertMac {
        #[arg(short, long)]
        bin: Option<String>,

        #[arg(short, long)]
        mac: Option<String>,
    },
    ClassTable {
        #[arg(short, long)]
        base_ip: Ipv4Addr,

        #[arg(short = 's', long)]
        num_subnets: u32,

        #[arg(short = 'H', long)]
        num_hosts: u32,
    },
    VlsmTable {
        #[arg(short, long)]
        base_ip: Ipv4Addr,

        #[arg(short = 'H', long, value_delimiter = ',', num_args = 1..)]
        hosts_per_subnet: Vec<u32>,
    },
}

#[derive(Parser, Debug)]
#[command(
    arg_required_else_help = true,
    styles = CLI_STYLE,
    author = crate_authors!(),
    version = crate_version!(),
    about = None,
    long_about = None,
)]
pub struct IpCli {
    #[command(subcommand)]
    pub(crate) command: IpCommands,
}
