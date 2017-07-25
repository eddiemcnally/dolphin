#[allow(dead_code)]
#[allow(non_camel_case_types)]


#[allow(dead_code)]
pub enum CastlePermissionBitMap {
    WK = 0x01,
    WQ = 0x02,
    BK = 0x04,
    BQ = 0x08,
}

// bitboard type
pub type CastlePermission = u8;


pub struct position{
	// pieces and squares
	board : Board,
	 // side to move
    side_to_move: piece::Colour,
    // the en passant square
    en_pass_sq: Square,
    // castle permissions
    castle_perm: CastlePermission,
}