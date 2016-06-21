
pub fn set_bit(brd: u64, bit: u8) -> u64 {
    brd | (0x01 << bit)
}

pub fn clear_bit(brd: u64, bit: u8) -> u64 {
    brd & (!(0x01 << bit))
}
