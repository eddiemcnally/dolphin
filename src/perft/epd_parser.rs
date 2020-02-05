use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub struct EpdRow {
    fen: String,
    depth_map: HashMap<u8, u64>,
}

// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ;D1 20 ;D2 400 ;D3 8902 ;D4 197281 ;D5 4865609 ;D6 119060324
pub fn extract_epd(file_name: String) -> Vec<EpdRow> {
    let mut retval = Vec::new();

    let lines = lines_from_file(file_name);
    for line in lines {
        let parsed = extract_row(line);
        retval.push(parsed);
    }

    return retval;
}

fn extract_row(row: String) -> EpdRow {
    let v: Vec<&str> = row.split(";").collect();

    assert_eq!(v.len(), 7); // FEN + 6-ply move counts

    let fen = v[0].trim();
    let mut map: HashMap<u8, u64> = HashMap::new();

    for elem in 1..7 {
        let (depth, count) = extract_ply_and_count(v[elem].to_string());

        map.insert(depth, count);
    }

    EpdRow {
        fen: fen.to_string(),
        depth_map: map,
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn extract_ply_and_count(ply_count: String) -> (u8, u64) {
    let v: Vec<&str> = ply_count.split(" ").collect();
    // extract the number from "D5"
    let d = &v[0][1..2];

    return (d.parse::<u8>().unwrap(), v[1].parse::<u64>().unwrap());
}

#[test]
fn parsed_epd_row_as_expected() {
    let epd = "4k2r/6K1/8/8/8/8/8/8 b k - 0 1 ;D1 12 ;D2 38 ;D3 564 ;D4 2219 ;D5 37735 ;D6 185867";

    let expected_fen = "4k2r/6K1/8/8/8/8/8/8 b k - 0 1";

    let row = extract_row(epd.to_string());

    assert_eq!(row.fen, expected_fen.to_string());

    // depth 1
    assert!(row.depth_map.get(&1u8).is_some());
    let d1_val = Some(&12u64);
    assert_eq!(row.depth_map.get(&1u8), d1_val);

    // depth 2
    assert!(row.depth_map.get(&2u8).is_some());
    let d2_val = Some(&38u64);
    assert_eq!(row.depth_map.get(&2u8), d2_val);

    // depth 3
    assert!(row.depth_map.get(&3u8).is_some());
    let d3_val = Some(&564u64);
    assert_eq!(row.depth_map.get(&3u8), d3_val);

    // depth 4
    assert!(row.depth_map.get(&4u8).is_some());
    let d4_val = Some(&2219u64);
    assert_eq!(row.depth_map.get(&4u8), d4_val);

    // depth 5
    assert!(row.depth_map.get(&5u8).is_some());
    let d5_val = Some(&37735u64);
    assert_eq!(row.depth_map.get(&5u8), d5_val);

    // depth 6
    assert!(row.depth_map.get(&6u8).is_some());
    let d6_val = Some(&185867u64);
    assert_eq!(row.depth_map.get(&6u8), d6_val);
}
