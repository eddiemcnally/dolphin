use num_enum::TryFromPrimitive;
use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn new(num: u8) -> File {
        match num {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Unexpected File"),
        }
    }

    pub const fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn can_add_one(self) -> bool {
        match self {
            File::A | File::B | File::C | File::D | File::E | File::F | File::G => true,
            File::H => false,
        }
    }

    pub fn can_add_two(self) -> bool {
        match self {
            File::A | File::B | File::C | File::D | File::E | File::F => true,
            File::G | File::H => false,
        }
    }

    pub fn can_subtract_one(self) -> bool {
        match self {
            File::B | File::C | File::D | File::E | File::F | File::G | File::H => true,
            File::A => false,
        }
    }

    pub fn can_subtract_two(self) -> bool {
        match self {
            File::C | File::D | File::E | File::F | File::G | File::H => true,
            File::A | File::B => false,
        }
    }

    pub fn add_one(self) -> File {
        match self {
            File::A => File::B,
            File::B => File::C,
            File::C => File::D,
            File::D => File::E,
            File::E => File::F,
            File::F => File::G,
            File::G => File::H,
            File::H => panic!("Invalid file"),
        }
    }

    pub fn subtract_one(self) -> File {
        match self {
            File::A => panic!("Invalid file"),
            File::B => File::A,
            File::C => File::B,
            File::D => File::C,
            File::E => File::D,
            File::F => File::E,
            File::G => File::F,
            File::H => File::G,
        }
    }

    pub fn add_two(self) -> File {
        match self {
            File::A => File::C,
            File::B => File::D,
            File::C => File::E,
            File::D => File::F,
            File::E => File::G,
            File::F => File::H,
            File::G | File::H => panic!("Invalid file"),
        }
    }

    pub fn subtract_two(self) -> File {
        match self {
            File::A | File::B => panic!("Invalid file"),
            File::C => File::A,
            File::D => File::B,
            File::E => File::C,
            File::F => File::D,
            File::G => File::E,
            File::H => File::F,
        }
    }

    pub fn from_char(file: char) -> Option<File> {
        match file {
            'a' => Some(File::A),
            'b' => Some(File::B),
            'c' => Some(File::C),
            'd' => Some(File::D),
            'e' => Some(File::E),
            'f' => Some(File::F),
            'g' => Some(File::G),
            'h' => Some(File::H),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match *self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
    pub fn iterator() -> Iter<'static, File> {
        static FILES: [File; 8] = [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ];
        FILES.iter()
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();

        debug_str.push_str(&format!("{}", self.to_char()));

        write!(f, "{}", debug_str)
    }
}

#[cfg(test)]
pub mod tests {
    use super::File;
    use std::collections::HashMap;

    #[test]
    pub fn file_as_u8() {
        assert!(File::A as u8 == 0);
        assert!(File::H as u8 == 7);
    }

    #[test]
    pub fn can_add_one() {
        assert!(File::A.can_add_one());
        assert!(File::B.can_add_one());
        assert!(File::C.can_add_one());
        assert!(File::D.can_add_one());
        assert!(File::E.can_add_one());
        assert!(File::F.can_add_one());
        assert!(File::G.can_add_one());
        assert!(!File::H.can_add_one());
    }

    #[test]
    pub fn can_add_two() {
        assert!(File::A.can_add_two());
        assert!(File::B.can_add_two());
        assert!(File::C.can_add_two());
        assert!(File::D.can_add_two());
        assert!(File::E.can_add_two());
        assert!(File::F.can_add_two());
        assert!(!File::G.can_add_two());
        assert!(!File::H.can_add_two());
    }

    #[test]
    pub fn can_subtract_one() {
        assert!(!File::A.can_subtract_one());
        assert!(File::B.can_subtract_one());
        assert!(File::C.can_subtract_one());
        assert!(File::D.can_subtract_one());
        assert!(File::E.can_subtract_one());
        assert!(File::F.can_subtract_one());
        assert!(File::G.can_subtract_one());
        assert!(File::H.can_subtract_one());
    }

    #[test]
    pub fn can_subtract_two() {
        assert!(!File::A.can_subtract_two());
        assert!(!File::B.can_subtract_two());
        assert!(File::C.can_subtract_two());
        assert!(File::D.can_subtract_two());
        assert!(File::E.can_subtract_two());
        assert!(File::F.can_subtract_two());
        assert!(File::G.can_subtract_two());
        assert!(File::H.can_subtract_two());
    }

    #[test]
    pub fn add_one() {
        assert_eq!(File::A.add_one(), File::B);
        assert_eq!(File::B.add_one(), File::C);
        assert_eq!(File::C.add_one(), File::D);
        assert_eq!(File::D.add_one(), File::E);
        assert_eq!(File::E.add_one(), File::F);
        assert_eq!(File::F.add_one(), File::G);
        assert_eq!(File::G.add_one(), File::H);
    }

    #[test]
    pub fn add_two() {
        assert_eq!(File::A.add_two(), File::C);
        assert_eq!(File::B.add_two(), File::D);
        assert_eq!(File::C.add_two(), File::E);
        assert_eq!(File::D.add_two(), File::F);
        assert_eq!(File::E.add_two(), File::G);
        assert_eq!(File::F.add_two(), File::H);
    }

    #[test]
    pub fn subract_one() {
        assert_eq!(File::B.subtract_one(), File::A);
        assert_eq!(File::C.subtract_one(), File::B);
        assert_eq!(File::D.subtract_one(), File::C);
        assert_eq!(File::E.subtract_one(), File::D);
        assert_eq!(File::F.subtract_one(), File::E);
        assert_eq!(File::G.subtract_one(), File::F);
        assert_eq!(File::H.subtract_one(), File::G);
    }

    #[test]
    pub fn subract_two() {
        assert_eq!(File::C.subtract_two(), File::A);
        assert_eq!(File::D.subtract_two(), File::B);
        assert_eq!(File::E.subtract_two(), File::C);
        assert_eq!(File::F.subtract_two(), File::D);
        assert_eq!(File::G.subtract_two(), File::E);
        assert_eq!(File::H.subtract_two(), File::F);
    }

    #[test]
    pub fn file_from_char() {
        let map = get_file_map();
        for (file, ch) in map {
            let f = File::from_char(ch);
            assert_eq!(f.unwrap(), file);
        }
    }

    #[test]
    pub fn file_to_char() {
        let map = get_file_map();
        for (file, ch) in map {
            let cc = file.to_char();
            assert_eq!(cc, ch);
        }
    }

    #[test]
    pub fn file_to_int() {
        assert_eq!(File::A.as_index(), 0);
        assert_eq!(File::B.as_index(), 1);
        assert_eq!(File::C.as_index(), 2);
        assert_eq!(File::D.as_index(), 3);
        assert_eq!(File::E.as_index(), 4);
        assert_eq!(File::F.as_index(), 5);
        assert_eq!(File::G.as_index(), 6);
        assert_eq!(File::H.as_index(), 7);
    }

    fn get_file_map() -> HashMap<File, char> {
        let mut map: HashMap<File, char> = HashMap::new();
        map.insert(File::A, 'a');
        map.insert(File::B, 'b');
        map.insert(File::C, 'c');
        map.insert(File::D, 'd');
        map.insert(File::E, 'e');
        map.insert(File::F, 'f');
        map.insert(File::G, 'g');
        map.insert(File::H, 'h');
        map
    }
}
