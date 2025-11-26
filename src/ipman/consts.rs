use clap::builder::Styles;
use clap::builder::styling::AnsiColor;

pub(crate) const BASE_2: u32 = 2;
pub(crate) const BASE_16: u32 = 16;

pub(crate) const CLI_STYLE: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Blue.on_default());

pub(crate) const MAC_HEX_PAIR_COUNT: usize = 6;
