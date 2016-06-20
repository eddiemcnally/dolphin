
fn set_bit(brd: &u64, bit: u8) {
    *brd = *brd | (u64) (0x01 << bit);
}

fn clear_bit(brd: &u64, bit: u8){
     *brd = *brd & (u64) (~(0x01 << bit));
}
