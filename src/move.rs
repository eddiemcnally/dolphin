pub type Move = u16;

// Bit representation as follows
// 		0000 0000 0011 1111		From square
//		0000 1111 1100 0000		To square
//		0001 0000 0000 0000		Promote to Knight
//		0010 0000 0000 0000		Promote to Bishop
//		0100 0000 0000 0000		Promote to Rook
//		1000 0000 0000 0000		Promote to Queen




#[cfg(test)]
mod tests {}
