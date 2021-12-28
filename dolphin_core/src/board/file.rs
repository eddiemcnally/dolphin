use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::fmt;
use std::slice::Iter;

#[derive(Eq, PartialEq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u64)]
pub enum File {
    FileA,
    FileB,
    FileC,
    FileD,
    FileE,
    FileF,
    FileG,
    FileH,
}

impl File {
    pub fn from_num(num: u64) -> Option<File> {
        let file = File::try_from(num);
        match file {
            Ok(file) => Some(file),
            _ => None,
        }
    }

    pub fn add_one(&self) -> Option<File> {
        let new_file = *self as u64 + 1;
        File::from_num(new_file)
    }

    pub fn subtract_one(&self) -> Option<File> {
        match self {
            File::FileA => None,
            _ => File::from_num(*self as u64 - 1),
        }
    }

    pub fn add_two(&self) -> Option<File> {
        File::from_num(*self as u64 + 2)
    }

    pub fn subtract_two(&self) -> Option<File> {
        match self {
            File::FileA | File::FileB => None,
            _ => File::from_num(*self as u64 - 2),
        }
    }

    pub fn from_char(file: char) -> Option<File> {
        match file {
            'a' => Some(File::FileA),
            'b' => Some(File::FileB),
            'c' => Some(File::FileC),
            'd' => Some(File::FileD),
            'e' => Some(File::FileE),
            'f' => Some(File::FileF),
            'g' => Some(File::FileG),
            'h' => Some(File::FileH),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            File::FileA => 'a',
            File::FileB => 'b',
            File::FileC => 'c',
            File::FileD => 'd',
            File::FileE => 'e',
            File::FileF => 'f',
            File::FileG => 'g',
            File::FileH => 'h',
        }
    }
    pub fn iterator() -> Iter<'static, File> {
        static FILES: [File; 8] = [
            File::FileA,
            File::FileB,
            File::FileC,
            File::FileD,
            File::FileE,
            File::FileF,
            File::FileG,
            File::FileH,
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
        assert!(File::FileA as u8 == 0);
        assert!(File::FileH as u8 == 7);
    }

    #[test]
    pub fn file_from_u8() {
        assert!(File::from_num(0) == Some(File::FileA));
        assert!(File::from_num(1) == Some(File::FileB));
        assert!(File::from_num(2) == Some(File::FileC));
        assert!(File::from_num(3) == Some(File::FileD));
        assert!(File::from_num(4) == Some(File::FileE));
        assert!(File::from_num(5) == Some(File::FileF));
        assert!(File::from_num(6) == Some(File::FileG));
        assert!(File::from_num(7) == Some(File::FileH));
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
        map.insert(File::FileA, 'a');
        map.insert(File::FileB, 'b');
        map.insert(File::FileC, 'c');
        map.insert(File::FileD, 'd');
        map.insert(File::FileE, 'e');
        map.insert(File::FileF, 'f');
        map.insert(File::FileG, 'g');
        map.insert(File::FileH, 'h');
        map
    }
}
