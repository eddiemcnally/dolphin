# dolphin

Chess engine in Rust

## Overview

I ran into a couple of seg faults with Kestrel (and K2) that was taking a lot of time (and pain) to get sorted.

I decided to leave it and start again in Rust.

Main features and current status:

- uses bitboards (u64) and occupancy bitmaps
- FEN parser
- move generation is coded and tested:
  - uses Hyperbola Quintessence for sliding piece attacks (see <https://www.chessprogramming.org/Hyperbola_Quintessence>)
  - uses occupancy masks for other pieces
- make_move/take-move is coded and tested
- perft results to depth=6 has been verified

To Do:

- Implement move evaluation
- Implement search
- UCI

