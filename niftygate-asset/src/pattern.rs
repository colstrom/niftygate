use regex::Regex;
use std::str::FromStr;

/// Some kind of pattern for matching text against
#[non_exhaustive]
pub enum Pattern {
  /// True if the text matches this exact string.
  String(String),
  /// True if the text starts with this prefix.
  Prefix(String),
  /// True if the text ends with this suffix.
  Suffix(String),
  /// True if the text matches this regular expression.
  Regex(Regex),
}

impl Pattern {
  pub fn matches(&self, text: &str) -> bool {
    match self {
      Self::String(pattern) => pattern.eq(text),
      Self::Prefix(pattern) => text.starts_with(pattern.as_str()),
      Self::Suffix(pattern) => text.ends_with(pattern.as_str()),
      Self::Regex(pattern) => pattern.is_match(text),
    }
  }
}

impl From<String> for Pattern {
  fn from(string: String) -> Self {
    Self::String(string)
  }
}

impl From<Regex> for Pattern {
  fn from(regex: Regex) -> Self {
    Self::Regex(regex)
  }
}

impl FromStr for Pattern {
  type Err = regex::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.starts_with('/') && s.ends_with('/') {
      Ok(Self::Regex(Regex::new(&s[1..s.len() - 1])?))
    } else {
      Ok(Self::String(s.to_string()))
    }
  }
}

pub(crate) trait FromPattern<T> {
  fn from_pattern(pattern: Pattern) -> T;
}
