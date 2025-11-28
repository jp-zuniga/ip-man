use colored::Colorize;

use crate::ipman::consts::COL_WIDTH;

use super::table::Subnet;

pub(crate) fn print_table(table: Vec<Subnet>) {
    println!();
    println!(
        "{:^w$} {:^w$} {:^w$} {:^w$} {:^w$}",
        "Subnet ID".yellow().bold(),
        "First Host".yellow().bold(),
        "Last Host".yellow().bold(),
        "Broadcast".yellow().bold(),
        "Subnet Mask".yellow().bold(),
        w = COL_WIDTH
    );

    println!("{}", "-".repeat(COL_WIDTH * 5 + 4).yellow());

    for s in table {
        println!(
            "{:^w$} {:^w$} {:^w$} {:^w$} {:^w$}",
            s.id,
            s.host_range.0,
            s.host_range.1,
            s.broadcast,
            s.mask,
            w = COL_WIDTH
        );
    }
}
