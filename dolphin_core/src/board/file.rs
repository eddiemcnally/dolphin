use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct File(u8);

impl File {
    pub const A: File = File(0);
    pub const B: File = File(1);
    pub const C: File = File(2);
    pub const D: File = File(3);
    pub const E: File = File(4);
    pub const F: File = File(5);
    pub const G: File = File(6);
    pub const H: File = File(7);

    pub fn new(num: u8) -> Option<File> {
        if num <= File::H.0 {
            return Some(File(num));
        }
        None
    }

    pub const fn as_index(self) -> usize {
        self.0 as usize
    }

    pub fn add_one(self) -> Option<File> {
        File::new(self.0 + 1)
    }

    pub fn subtract_one(self) -> Option<File> {
        if self.0 >= File::B.0 {
            return File::new(self.0 - 1);
        }
        None
    }

    pub fn add_two(self) -> Option<File> {
        File::new(self.0 + 2)
    }

    pub fn subtract_two(self) -> Option<File> {
        if self.0 >= File::C.0 {
            return File::new(self.0 - 2);
        }
        None
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
    pub fn to_char(self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
            _ => '-',
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
