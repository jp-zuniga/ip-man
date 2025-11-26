use super::consts::{BASE_16, BASE_2, MAC_HEX_PAIR_COUNT};

pub(crate) struct MacAddr {
    pub(crate) hexes: [u8; MAC_HEX_PAIR_COUNT],
}

impl MacAddr {
    pub fn from_bin(bin_mac: &str) -> Result<Self, ()> {
        let split_pairs: Vec<&str> = bin_mac.split(":").collect();

        let mut new_pairs = [0; 6];

        for (idx, pair) in split_pairs.iter().enumerate() {
            let conversion = u8::from_str_radix(pair, BASE_2);

            if conversion.is_err() {
                return Err(());
            }

            new_pairs[idx] = conversion.expect("err should have already been returned");
        }

        Ok(MacAddr { hexes: new_pairs })
    }

    pub fn from_str(mac: String) -> Result<Self, ()> {
        let split: Vec<&str> = mac.split(":").collect();

        let mut hexes = [0u8; 6];

        for (idx, pair) in split.iter().enumerate() {
            let conversion = u8::from_str_radix(pair, BASE_16);

            if conversion.is_err() {
                return Err(());
            }

            hexes[idx] = conversion.expect("err should have already been returned");
        }

        Ok(MacAddr { hexes })
    }

    pub fn to_bin(&self) -> String {
        let mut bin = String::new();

        for hex in self.hexes {
            let join = if &hex != self.hexes.iter().last().unwrap() {
                ":"
            } else {
                ""
            };

            bin.push_str(
                format!("{}{hex:b}{join}", "0".repeat(hex.leading_zeros() as usize)).as_str(),
            );
        }

        bin
    }

    pub fn to_str(&self) -> String {
        let mut self_str = String::new();

        for hex in self.hexes {
            let join = if &hex != self.hexes.iter().last().unwrap() {
                ":"
            } else {
                ""
            };

            self_str.push_str(format!("{}{join}", format!("{hex:x}").to_uppercase()).as_str());
        }

        self_str
    }
}
