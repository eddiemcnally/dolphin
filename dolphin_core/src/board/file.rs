use crate::core::types::ToInt;
use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct File(u8);

impl ToInt for File {
    fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl File {
    pub const A: File = File(0);
    pub const B: File = File(1);
    pub const C: File = File(2);
    pub const D: File = File(3);
    pub const E: File = File(4);
    pub const F: File = File(5);
    pub const G: File = File(6);
    pub const H: File = File(7);

    pub const fn new(num: u8) -> Option<File> {
        if num <= File::H.0 {
            return Some(File(num));
        }
        None
    }

    pub fn add_one(&self) -> Option<File> {
        File::new(self.0 + 1)
    }

    pub fn subtract_one(&self) -> Option<File> {
        match *self {
            File::A => None,
            _ => File::new(self.0 - 1),
        }
    }

    pub fn add_two(&self) -> Option<File> {
        File::new(self.0 + 2)
    }

    pub fn subtract_two(&self) -> Option<File> {
        match *self {
            File::A | File::B => None,
            _ => File::new(self.0 - 2),
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
            _ => panic!("Invalid File {}", *self),
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
        assert!(File::A.0 as u8 == 0);
        assert!(File::H.0 as u8 == 7);
    }

    #[test]
    pub fn file_from_u8() {
        assert!(File::new(0) == Some(File::A));
        assert!(File::new(1) == Some(File::B));
        assert!(File::new(2) == Some(File::C));
        assert!(File::new(3) == Some(File::D));
        assert!(File::new(4) == Some(File::E));
        assert!(File::new(5) == Some(File::F));
        assert!(File::new(6) == Some(File::G));
        assert!(File::new(7) == Some(File::H));
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
