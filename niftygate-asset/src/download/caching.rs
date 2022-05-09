use super::Downloader;
use crate::compatibility::Path;
use crate::prelude::http::{Body, Error, Response, StatusCode, Url};
use crate::storage::Storage;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
  #[error("failed to download")]
  DownloadFailed,
  #[error("failed to read from cache")]
  ReadFailed,
  #[error("failed to write to cache")]
  WriteFailed,
}

pub struct CachingDownloader<S, D>
where
  S: Storage,
  D: Downloader,
{
  cache: S,
  downloader: D,
  origin: Url,
  hostnames_in_cache_path: bool,
}

impl<D, S> CachingDownloader<S, D>
where
  D: Downloader,
  S: Storage,
{
  fn path(&self, url: &Url) -> Result<<S as Storage>::Path, <S as Storage>::Error> {
    Ok(self.cache.root().join(url.as_ref())?)
  }

  fn read(&self, url: &Url) -> Result<Option<Vec<u8>>, <S as Storage>::Error> {
    match self.path(url)?.open_file_if_exists()? {
      None => Ok(None),
      Some(mut reader) => {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(Some(buf))
      }
    }
  }

  fn write(&self, url: &Url, buf: Vec<u8>) -> Result<Vec<u8>, <S as Storage>::Error> {
    let mut writer = self.path(url)?.create_file_with_parents()?;
    writer.write_all(&buf)?;
    writer.flush()?;
    Ok(buf)
  }

  async fn fetch(&self, url: &Url) -> Result<Vec<u8>, Error> {
    Ok(self.downloader.download(url).await?.body_bytes().await?)
  }

  async fn get(&self, url: &Url) -> Result<Response, CacheError> {
    let bytes = match self.read(url).map_err(|_| CacheError::ReadFailed)? {
      Some(bytes) => bytes,
      None => {
        let bytes = self
          .fetch(url)
          .await
          .map_err(|_| CacheError::DownloadFailed)?;
        self
          .write(url, bytes)
          .map_err(|_| CacheError::WriteFailed)?
      }
    };

    let body = Body::from_bytes(bytes);
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(body);
    Ok(response)
  }
}

#[async_trait]
impl<S, D> Downloader for CachingDownloader<S, D>
where
  S: Storage + Send + Sync,
  D: Downloader,
{
  async fn download(&self, url: &Url) -> Result<Response, Error> {
    Ok(self.get(url).await?)
  }
}
