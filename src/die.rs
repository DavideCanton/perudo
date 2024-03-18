use std::fmt;

const LAMA_VALUE: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Die(u8);

impl Die {
    pub fn new(val: u8) -> Option<Self> {
        if (1..=6).contains(&val) {
            Some(Die(val))
        } else {
            None
        }
    }

    pub fn new_lama() -> Self {
        Die(LAMA_VALUE)
    }

    pub fn matches_value(&self, n: u8, is_palifico: bool) -> bool {
        if is_palifico {
            n == self.0
        } else {
            n == self.0 || self.is_lama()
        }
    }

    pub fn is_lama(&self) -> bool {
        self.0 == LAMA_VALUE
    }

    pub fn get_value(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for Die {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
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
        let d = Die::new(4).unwrap();
        assert_eq!(d.get_value(), 4);

        let d2 = Die::new_lama();
        assert_eq!(d2.get_value(), LAMA_VALUE);
    }

    #[test]
    fn test_die_invalid() {
        for i in [0, 7, 255] {
            let d = Die::new(i);
            assert!(d.is_none());
        }
    }

    #[test]
    fn test_die_matches() {
        let d = Die::new(4).unwrap();
        assert!(!d.is_lama());
        assert!(d.matches_value(4, false));
        assert!(d.matches_value(4, true));
        assert!(!d.matches_value(5, false));
        assert!(!d.matches_value(5, true));
    }

    #[test]
    fn test_die_lama_matches() {
        let d = Die::new_lama();
        assert!(d.is_lama());
        assert!(d.matches_value(LAMA_VALUE, false));
        assert!(d.matches_value(LAMA_VALUE, true));
        assert!(d.matches_value(2, false));
        assert!(!d.matches_value(2, true));
    }
}
