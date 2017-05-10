use std::fmt;

/// Hardware (Ethernet) address
pub struct HwAddr<'a> {
    pub hw: &'a [u8],
}

impl<'a> HwAddr<'a> {
    pub fn new(s: &'a [u8]) -> HwAddr<'a> {
        HwAddr {
            hw: s,
        }
    }
}

impl<'a> fmt::Display for HwAddr<'a> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let s = self.hw.iter().fold(
            String::new(),
            |acc, &b| {
                (if acc.len()>0 {acc + ":"} else {acc}) + &format!("{:2x}",b)
            }
        );
        return write!(out, "{}", s);
    }
}



