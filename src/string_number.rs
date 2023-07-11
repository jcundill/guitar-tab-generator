use anyhow::{anyhow, Result};
use std::fmt;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringNumber(u8);
impl StringNumber {
    pub fn new(string_number: u8) -> Result<Self> {
        const MAX_NUM_STRINGS: u8 = 12;
        match string_number {
            0 => Err(anyhow!("A guitar cannot have a string number of zero (0). Guitar string numbering commences at one (1).")),
            1..=MAX_NUM_STRINGS => Ok(StringNumber(string_number)),
            _ => Err(anyhow!("The string number ({string_number}) is too high. The maximum is {MAX_NUM_STRINGS}."))

        }
    }
}
#[cfg(test)]
mod test_create_string_number {
    use super::*;
    #[test]
    fn valid_simple() {
        assert!(StringNumber::new(1).is_ok());
    }
    #[test]
    fn invalid_zero() {
        let expected_error_string = "A guitar cannot have a string number of zero (0). Guitar string numbering commences at one (1).";
        let error = StringNumber::new(0).unwrap_err();
        assert_eq!(format!("{error}"), expected_error_string);
    }
    #[test]
    fn invalid_too_high() {
        let expected_error_string = "The string number (15) is too high. The maximum is 12.";
        let error = StringNumber::new(15).unwrap_err();
        assert_eq!(format!("{error}"), expected_error_string);
    }
}

impl fmt::Debug for StringNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.0)
        let string_number = self.0;
        let string_pitch_letter = match string_number {
            1 => "1_e".to_owned(),
            2 => "2_B".to_owned(),
            3 => "3_G".to_owned(),
            4 => "4_D".to_owned(),
            5 => "5_A".to_owned(),
            6 => "6_E".to_owned(),
            string_number => string_number.to_string(),
        };
        write!(f, "{}", string_pitch_letter)
    }
}
