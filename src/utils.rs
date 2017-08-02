

use square::rank::Rank;
use square::file::File;
use square::Square;
use std::collections::HashMap;
use std::collections::HashSet;




pub fn get_square_set() -> HashSet<Square> {
    let mut set = HashSet::new();

    set.insert(Square::a1);
    set.insert(Square::a2);
    set.insert(Square::a3);
    set.insert(Square::a4);
    set.insert(Square::a5);
    set.insert(Square::a6);
    set.insert(Square::a7);
    set.insert(Square::a8);

    set.insert(Square::b1);
    set.insert(Square::b2);
    set.insert(Square::b3);
    set.insert(Square::b4);
    set.insert(Square::b5);
    set.insert(Square::b6);
    set.insert(Square::b7);
    set.insert(Square::b8);

    set.insert(Square::c1);
    set.insert(Square::c2);
    set.insert(Square::c3);
    set.insert(Square::c4);
    set.insert(Square::c5);
    set.insert(Square::c6);
    set.insert(Square::c7);
    set.insert(Square::c8);

    set.insert(Square::d1);
    set.insert(Square::d2);
    set.insert(Square::d3);
    set.insert(Square::d4);
    set.insert(Square::d5);
    set.insert(Square::d6);
    set.insert(Square::d7);
    set.insert(Square::d8);

    set.insert(Square::e1);
    set.insert(Square::e2);
    set.insert(Square::e3);
    set.insert(Square::e4);
    set.insert(Square::e5);
    set.insert(Square::e6);
    set.insert(Square::e7);
    set.insert(Square::e8);

    set.insert(Square::f1);
    set.insert(Square::f2);
    set.insert(Square::f3);
    set.insert(Square::f4);
    set.insert(Square::f5);
    set.insert(Square::f6);
    set.insert(Square::f7);
    set.insert(Square::f8);

    set.insert(Square::g1);
    set.insert(Square::g2);
    set.insert(Square::g3);
    set.insert(Square::g4);
    set.insert(Square::g5);
    set.insert(Square::g6);
    set.insert(Square::g7);
    set.insert(Square::g8);

    set.insert(Square::h1);
    set.insert(Square::h2);
    set.insert(Square::h3);
    set.insert(Square::h4);
    set.insert(Square::h5);
    set.insert(Square::h6);
    set.insert(Square::h7);
    set.insert(Square::h8);

    return set;
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
