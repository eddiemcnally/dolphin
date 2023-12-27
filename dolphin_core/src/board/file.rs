use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
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
    pub fn new(num: u8) -> Option<File> {
        match num {
            0 => Some(File::A),
            1 => Some(File::B),
            2 => Some(File::C),
            3 => Some(File::D),
            4 => Some(File::E),
            5 => Some(File::F),
            6 => Some(File::G),
            7 => Some(File::H),
            _ => None,
        }
    }

    pub const fn as_index(&self) -> usize {
        *self as usize
    }

    pub fn add_one(self) -> Option<File> {
        match self {
            File::A => Some(File::B),
            File::B => Some(File::C),
            File::C => Some(File::D),
            File::D => Some(File::E),
            File::E => Some(File::F),
            File::F => Some(File::G),
            File::G => Some(File::H),
            File::H => None,
        }
    }

    pub fn subtract_one(self) -> Option<File> {
        match self {
            File::A => None,
            File::B => Some(File::A),
            File::C => Some(File::B),
            File::D => Some(File::C),
            File::E => Some(File::D),
            File::F => Some(File::E),
            File::G => Some(File::F),
            File::H => Some(File::G),
        }
    }

    pub fn add_two(self) -> Option<File> {
        match self {
            File::A => Some(File::C),
            File::B => Some(File::D),
            File::C => Some(File::E),
            File::D => Some(File::F),
            File::E => Some(File::G),
            File::F => Some(File::H),
            File::G | File::H => None,
        }
    }

    pub fn subtract_two(self) -> Option<File> {
        match self {
            File::A | File::B => None,
            File::C => Some(File::A),
            File::D => Some(File::B),
            File::E => Some(File::C),
            File::F => Some(File::D),
            File::G => Some(File::E),
            File::H => Some(File::F),
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
    pub fn add_one() {
        assert_eq!(File::A.add_one(), Some(File::B));
        assert_eq!(File::B.add_one(), Some(File::C));
        assert_eq!(File::C.add_one(), Some(File::D));
        assert_eq!(File::D.add_one(), Some(File::E));
        assert_eq!(File::E.add_one(), Some(File::F));
        assert_eq!(File::F.add_one(), Some(File::G));
        assert_eq!(File::G.add_one(), Some(File::H));
        assert_eq!(File::H.add_one(), None);
    }

    #[test]
    pub fn add_two() {
        assert_eq!(File::A.add_two(), Some(File::C));
        assert_eq!(File::B.add_two(), Some(File::D));
        assert_eq!(File::C.add_two(), Some(File::E));
        assert_eq!(File::D.add_two(), Some(File::F));
        assert_eq!(File::E.add_two(), Some(File::G));
        assert_eq!(File::F.add_two(), Some(File::H));
        assert_eq!(File::G.add_two(), None);
        assert_eq!(File::H.add_two(), None);
    }

    #[test]
    pub fn subract_one() {
        assert_eq!(File::A.subtract_one(), None);
        assert_eq!(File::B.subtract_one(), Some(File::A));
        assert_eq!(File::C.subtract_one(), Some(File::B));
        assert_eq!(File::D.subtract_one(), Some(File::C));
        assert_eq!(File::E.subtract_one(), Some(File::D));
        assert_eq!(File::F.subtract_one(), Some(File::E));
        assert_eq!(File::G.subtract_one(), Some(File::F));
        assert_eq!(File::H.subtract_one(), Some(File::G));
    }

    #[test]
    pub fn subract_two() {
        assert_eq!(File::A.subtract_two(), None);
        assert_eq!(File::B.subtract_two(), None);
        assert_eq!(File::C.subtract_two(), Some(File::A));
        assert_eq!(File::D.subtract_two(), Some(File::B));
        assert_eq!(File::E.subtract_two(), Some(File::C));
        assert_eq!(File::F.subtract_two(), Some(File::D));
        assert_eq!(File::G.subtract_two(), Some(File::E));
        assert_eq!(File::H.subtract_two(), Some(File::F));
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
