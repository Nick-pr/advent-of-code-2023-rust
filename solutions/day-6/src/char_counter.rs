pub struct CharCounter([u32; 26]);

impl CharCounter {
    pub fn new() -> Self {
        return Self([0; 26]);
    }

    pub fn inc(&mut self, c: &char) {
        self.set(c, self.get(c) + 1);
    }

    pub fn dec(&mut self, c: &char) {
        let count = self.get(c);
        if count > 0 {
            self.set(c, count - 1);
        }
    }

    pub fn get(&self, c: &char) -> u32 {
        self.0[(*c as u8 - 97) as usize]
    }

    fn set(&mut self, c: &char, value: u32) {
        self.0[(*c as u8 - 97) as usize] = value;
    }

    pub fn has_duplicates(&self) -> bool {
        for i in self.0.iter() {
            if *i > 1 {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::CharCounter;

    #[test]
    fn test_charcounter() {
        let mut char_counter = CharCounter::new();

        char_counter.inc(&'c');
        assert_eq!(char_counter.get(&'c'), 1);

        char_counter.inc(&'y');
        char_counter.inc(&'y');
        assert_eq!(char_counter.get(&'y'), 2);

        char_counter.dec(&'c');
        assert_eq!(char_counter.get(&'c'), 0);

        assert_eq!(char_counter.get(&'h'), 0);

        assert_eq!(char_counter.has_duplicates(), true);
        char_counter.dec(&'y');
        assert_eq!(char_counter.has_duplicates(), false);
    }
}
