use crate::{traits::Length, MaxLengthErrorParams};

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
pub trait ValidateMaxLength {
    fn validate(&self, max_length: usize) -> Result<(), MaxLengthErrorParams>;
}

impl<T> ValidateMaxLength for T
where
    T: Length + ?Sized,
{
    fn validate(&self, max_length: usize) -> Result<(), MaxLengthErrorParams> {
        if max_length >= self.length() {
            Ok(())
        } else {
            Err(MaxLengthErrorParams::new(max_length))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_max_length_ascii_is_true() {
        assert!(ValidateMaxLength::validate("abcde", 5).is_ok());
        assert!(ValidateMaxLength::validate("abcde", 6).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_unicode_is_true() {
        assert!(ValidateMaxLength::validate("a̐éö̲", 3).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_japanese_is_true() {
        assert!(ValidateMaxLength::validate("あ堯", 2).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_emoji_is_true() {
        assert!(ValidateMaxLength::validate("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦", 5).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_string_type() {
        assert!(ValidateMaxLength::validate(&String::from("abcde"), 5).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_cow_str_type() {
        assert!(ValidateMaxLength::validate(&Cow::from("abcde"), 5).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_vec_u8_type() {
        assert!(ValidateMaxLength::validate(&"abcde".as_bytes().to_vec(), 5).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_vec_char_type() {
        assert!(ValidateMaxLength::validate(&vec!['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_u8_array_type() {
        assert!(ValidateMaxLength::validate("abcde".as_bytes(), 5).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_char_array_type() {
        assert!(ValidateMaxLength::validate(&['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_os_str_type() {
        assert!(ValidateMaxLength::validate(OsStr::new("fo�o"), 4).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_os_string_type() {
        assert!(ValidateMaxLength::validate(&OsString::from("fo�o"), 4).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_path_type() {
        assert!(ValidateMaxLength::validate(&Path::new("./foo/bar.txt"), 13).is_ok());
    }

    #[test]
    fn test_validate_string_max_length_path_buf_type() {
        assert!(ValidateMaxLength::validate(&PathBuf::from("./foo/bar.txt"), 13).is_ok());
    }
}
