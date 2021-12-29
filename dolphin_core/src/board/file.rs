use std::fmt;
use std::slice::Iter;

use super::types::ToInt;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct File(u8);

pub const FILE_A: File = File(0);
pub const FILE_B: File = File(1);
pub const FILE_C: File = File(2);
pub const FILE_D: File = File(3);
pub const FILE_E: File = File(4);
pub const FILE_F: File = File(5);
pub const FILE_G: File = File(6);
pub const FILE_H: File = File(7);

impl ToInt for File {
    fn to_u8(&self) -> u8 {
        self.0 as u8
    }

    fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl File {
    pub fn new(num: u8) -> Option<File> {
        if num <= FILE_H.0 {
            return Some(File(num));
        }
        None
    }

    pub fn add_one(&self) -> Option<File> {
        File::new(self.0 + 1)
    }

    pub fn subtract_one(&self) -> Option<File> {
        match *self {
            FILE_A => None,
            _ => File::new(self.0 - 1),
        }
    }

    pub fn add_two(&self) -> Option<File> {
        File::new(self.0 + 2)
    }

    pub fn subtract_two(&self) -> Option<File> {
        match *self {
            FILE_A | FILE_B => None,
            _ => File::new(self.0 - 2),
        }
    }

    pub fn from_char(file: char) -> Option<File> {
        match file {
            'a' => Some(FILE_A),
            'b' => Some(FILE_B),
            'c' => Some(FILE_C),
            'd' => Some(FILE_D),
            'e' => Some(FILE_E),
            'f' => Some(FILE_F),
            'g' => Some(FILE_G),
            'h' => Some(FILE_H),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match *self {
            FILE_A => 'a',
            FILE_B => 'b',
            FILE_C => 'c',
            FILE_D => 'd',
            FILE_E => 'e',
            FILE_F => 'f',
            FILE_G => 'g',
            FILE_H => 'h',
            _ => panic!("Invalid File {}", *self),
        }
    }
    pub fn iterator() -> Iter<'static, File> {
        static FILES: [File; 8] = [
            FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
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
    use crate::board::file::*;
    use std::collections::HashMap;

    #[test]
    pub fn file_as_u8() {
        assert!(FILE_A.0 as u8 == 0);
        assert!(FILE_H.0 as u8 == 7);
    }

    #[test]
    pub fn file_from_u8() {
        assert!(File::new(0) == Some(FILE_A));
        assert!(File::new(1) == Some(FILE_B));
        assert!(File::new(2) == Some(FILE_C));
        assert!(File::new(3) == Some(FILE_D));
        assert!(File::new(4) == Some(FILE_E));
        assert!(File::new(5) == Some(FILE_F));
        assert!(File::new(6) == Some(FILE_G));
        assert!(File::new(7) == Some(FILE_H));
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
        map.insert(FILE_A, 'a');
        map.insert(FILE_B, 'b');
        map.insert(FILE_C, 'c');
        map.insert(FILE_D, 'd');
        map.insert(FILE_E, 'e');
        map.insert(FILE_F, 'f');
        map.insert(FILE_G, 'g');
        map.insert(FILE_H, 'h');
        map
    }
}
