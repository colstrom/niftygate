use super::super::compatibility::Path;
use regex::Regex;

pub(crate) fn is_file<P>(result: &Result<P, P::Error>) -> bool
where
  P: Path,
{
  result
    .as_ref()
    .map(|path| path.is_file().unwrap_or(true))
    .unwrap_or(true)
}

pub(crate) fn is_match<'pattern, P>(
  pattern: &'pattern Regex,
) -> impl FnMut(&Result<P, P::Error>) -> bool + 'pattern
where
  P: Path,
{
  move |result: &Result<P, P::Error>| result.as_ref().map_or(true, |path| path.is_match(pattern))
}

pub(crate) fn is_match_optional<'pattern, P>(
  pattern: Option<&'pattern Regex>,
) -> impl FnMut(&Result<P, P::Error>) -> bool + 'pattern
where
  P: Path,
{
  move |result: &Result<P, P::Error>| {
    pattern.map_or(true, |pattern| {
      result.as_ref().map_or(true, |path| path.is_match(pattern))
    })
  }
}
