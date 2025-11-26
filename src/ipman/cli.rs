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
    Table {
        #[arg(short, long)]
        subnet_id: Ipv4Addr,

        #[arg(short, long)]
        num_subnets: u32,

        #[arg(short, long)]
        num_hosts: u32,
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
