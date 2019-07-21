use board::square::file::File;
use board::square::rank::Rank;
use board::square::Square;
use std::collections::HashMap;
use std::vec::Vec;

// TODO : move into square.rs
pub fn get_ordered_square_list_by_file() -> Vec<Square> {
    let mut list: Vec<Square> = Vec::new();

    list.push(Square::a1);
    list.push(Square::b1);
    list.push(Square::c1);
    list.push(Square::d1);
    list.push(Square::e1);
    list.push(Square::f1);
    list.push(Square::g1);
    list.push(Square::h1);

    list.push(Square::a2);
    list.push(Square::b2);
    list.push(Square::c2);
    list.push(Square::d2);
    list.push(Square::e2);
    list.push(Square::f2);
    list.push(Square::g2);
    list.push(Square::h2);

    list.push(Square::a3);
    list.push(Square::b3);
    list.push(Square::c3);
    list.push(Square::d3);
    list.push(Square::e3);
    list.push(Square::f3);
    list.push(Square::g3);
    list.push(Square::h3);

    list.push(Square::a4);
    list.push(Square::b4);
    list.push(Square::c4);
    list.push(Square::d4);
    list.push(Square::e4);
    list.push(Square::f4);
    list.push(Square::g4);
    list.push(Square::h4);

    list.push(Square::a5);
    list.push(Square::b5);
    list.push(Square::c5);
    list.push(Square::d5);
    list.push(Square::e5);
    list.push(Square::f5);
    list.push(Square::g5);
    list.push(Square::h5);

    list.push(Square::a6);
    list.push(Square::b6);
    list.push(Square::c6);
    list.push(Square::d6);
    list.push(Square::e6);
    list.push(Square::f6);
    list.push(Square::g6);
    list.push(Square::h6);

    list.push(Square::a7);
    list.push(Square::b7);
    list.push(Square::c7);
    list.push(Square::d7);
    list.push(Square::e7);
    list.push(Square::f7);
    list.push(Square::g7);
    list.push(Square::h7);

    list.push(Square::a8);
    list.push(Square::b8);
    list.push(Square::c8);
    list.push(Square::d8);
    list.push(Square::e8);
    list.push(Square::f8);
    list.push(Square::g8);
    list.push(Square::h8);

    return list;
}

pub fn get_square_rank_file_map() -> HashMap<Square, (Rank, File)> {
    let mut map: HashMap<Square, (Rank, File)> = HashMap::new();

    map.insert(Square::a1, (Rank::Rank1, File::FileA));
    map.insert(Square::a2, (Rank::Rank2, File::FileA));
    map.insert(Square::a3, (Rank::Rank3, File::FileA));
    map.insert(Square::a4, (Rank::Rank4, File::FileA));
    map.insert(Square::a5, (Rank::Rank5, File::FileA));
    map.insert(Square::a6, (Rank::Rank6, File::FileA));
    map.insert(Square::a7, (Rank::Rank7, File::FileA));
    map.insert(Square::a8, (Rank::Rank8, File::FileA));

    map.insert(Square::b1, (Rank::Rank1, File::FileB));
    map.insert(Square::b2, (Rank::Rank2, File::FileB));
    map.insert(Square::b3, (Rank::Rank3, File::FileB));
    map.insert(Square::b4, (Rank::Rank4, File::FileB));
    map.insert(Square::b5, (Rank::Rank5, File::FileB));
    map.insert(Square::b6, (Rank::Rank6, File::FileB));
    map.insert(Square::b7, (Rank::Rank7, File::FileB));
    map.insert(Square::b8, (Rank::Rank8, File::FileB));

    map.insert(Square::c1, (Rank::Rank1, File::FileC));
    map.insert(Square::c2, (Rank::Rank2, File::FileC));
    map.insert(Square::c3, (Rank::Rank3, File::FileC));
    map.insert(Square::c4, (Rank::Rank4, File::FileC));
    map.insert(Square::c5, (Rank::Rank5, File::FileC));
    map.insert(Square::c6, (Rank::Rank6, File::FileC));
    map.insert(Square::c7, (Rank::Rank7, File::FileC));
    map.insert(Square::c8, (Rank::Rank8, File::FileC));

    map.insert(Square::d1, (Rank::Rank1, File::FileD));
    map.insert(Square::d2, (Rank::Rank2, File::FileD));
    map.insert(Square::d3, (Rank::Rank3, File::FileD));
    map.insert(Square::d4, (Rank::Rank4, File::FileD));
    map.insert(Square::d5, (Rank::Rank5, File::FileD));
    map.insert(Square::d6, (Rank::Rank6, File::FileD));
    map.insert(Square::d7, (Rank::Rank7, File::FileD));
    map.insert(Square::d8, (Rank::Rank8, File::FileD));

    map.insert(Square::e1, (Rank::Rank1, File::FileE));
    map.insert(Square::e2, (Rank::Rank2, File::FileE));
    map.insert(Square::e3, (Rank::Rank3, File::FileE));
    map.insert(Square::e4, (Rank::Rank4, File::FileE));
    map.insert(Square::e5, (Rank::Rank5, File::FileE));
    map.insert(Square::e6, (Rank::Rank6, File::FileE));
    map.insert(Square::e7, (Rank::Rank7, File::FileE));
    map.insert(Square::e8, (Rank::Rank8, File::FileE));

    map.insert(Square::f1, (Rank::Rank1, File::FileF));
    map.insert(Square::f2, (Rank::Rank2, File::FileF));
    map.insert(Square::f3, (Rank::Rank3, File::FileF));
    map.insert(Square::f4, (Rank::Rank4, File::FileF));
    map.insert(Square::f5, (Rank::Rank5, File::FileF));
    map.insert(Square::f6, (Rank::Rank6, File::FileF));
    map.insert(Square::f7, (Rank::Rank7, File::FileF));
    map.insert(Square::f8, (Rank::Rank8, File::FileF));

    map.insert(Square::g1, (Rank::Rank1, File::FileG));
    map.insert(Square::g2, (Rank::Rank2, File::FileG));
    map.insert(Square::g3, (Rank::Rank3, File::FileG));
    map.insert(Square::g4, (Rank::Rank4, File::FileG));
    map.insert(Square::g5, (Rank::Rank5, File::FileG));
    map.insert(Square::g6, (Rank::Rank6, File::FileG));
    map.insert(Square::g7, (Rank::Rank7, File::FileG));
    map.insert(Square::g8, (Rank::Rank8, File::FileG));

    map.insert(Square::h1, (Rank::Rank1, File::FileH));
    map.insert(Square::h2, (Rank::Rank2, File::FileH));
    map.insert(Square::h3, (Rank::Rank3, File::FileH));
    map.insert(Square::h4, (Rank::Rank4, File::FileH));
    map.insert(Square::h5, (Rank::Rank5, File::FileH));
    map.insert(Square::h6, (Rank::Rank6, File::FileH));
    map.insert(Square::h7, (Rank::Rank7, File::FileH));
    map.insert(Square::h8, (Rank::Rank8, File::FileH));

    return map;
}
