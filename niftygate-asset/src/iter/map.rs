use super::super::compatibility::Path;

pub(crate) fn with_metadata<P>(result: Result<P, P::Error>) -> Result<(P, P::Metadata), P::Error>
where
  P: Path,
{
  result.and_then(|path| path.metadata().map(|metadata| (path, metadata)))
}

pub(crate) fn metadata<P>(result: Result<P, P::Error>) -> Result<P::Metadata, P::Error>
where
  P: Path,
{
  result.and_then(|path| path.metadata())
}

pub(crate) fn with_filesize<P>(result: Result<P, P::Error>) -> Result<(P, u64), P::Error>
where
  P: Path,
{
  result.and_then(|path| path.filesize().map(|filesize| (path, filesize)))
}

pub(crate) fn filesize<P>(result: Result<P, P::Error>) -> Result<u64, P::Error>
where
  P: Path,
{
  result.and_then(|path| path.filesize())
}
