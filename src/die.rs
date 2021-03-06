use std::fmt;

const LAMA_VALUE: i32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Die {
    val: i32,
}

impl Die {
    pub fn new(val: i32) -> Self {
        if Die::is_lama_value(val) {
            Die::new_lama()
        } else {
            Die::create_die(val)
        }
    }

    pub fn new_lama() -> Self {
        Die::create_die(LAMA_VALUE)
    }

    fn create_die(val: i32) -> Self {
        Die { val }
    }

    pub fn matches_value(self, n: i32, is_palifico: bool) -> bool {
        if is_palifico {
            n == self.val
        } else {
            n == self.val || self.is_lama()
        }
    }

    pub fn is_lama(self) -> bool {
        Die::is_lama_value(self.val)
    }

    pub fn get_value(self) -> i32 {
        self.val
    }

    pub fn is_lama_value(val: i32) -> bool {
        val == LAMA_VALUE
    }
}

impl fmt::Display for Die {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.val {
            LAMA_VALUE => write!(fmt, "Lama"),
            n => write!(fmt, "{}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Die, LAMA_VALUE};

    #[test]
    fn test_die_val() {
        let d = Die::new(4);
        assert_eq!(d.val, 4);
    }

    #[test]
    fn test_die_matches() {
        let d = Die::new(4);
        assert!(d.matches_value(4, false));
        assert!(d.matches_value(4, true));
        assert!(!d.matches_value(5, false));
        assert!(!d.matches_value(5, true));
    }

    #[test]
    fn test_die_lama_matches() {
        let d = Die::new_lama();
        assert!(d.matches_value(LAMA_VALUE, false));
        assert!(d.matches_value(LAMA_VALUE, true));
        assert!(d.matches_value(2, false));
        assert!(!d.matches_value(2, true));
    }
}
