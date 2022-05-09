use super::Downloader;
use crate::prelude::http::{Body, Error, Response, StatusCode, Url};
use crate::storage::{DirectStorage, Storage};
use async_std::fs::File;
use async_std::io::BufReader;
use async_trait::async_trait;

#[derive(Clone, Debug, Default)]
pub struct LocalDownloader<T: Storage> {
  storage: T,
}

#[async_trait]
impl Downloader for LocalDownloader<DirectStorage> {
  async fn download(&self, url: &Url) -> Result<Response, Error> {
    let url = url.as_ref();
    let path = self.storage.root().join(url);

    if !path.exists() {
      let response = Response::new(StatusCode::NotFound);
      return Ok(response);
    }

    let file = File::open(path)
      .await
      .map_err(|error| Error::new(StatusCode::InternalServerError, error))?;
    let reader = BufReader::new(file);
    let body = Body::from_reader(reader, None);
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(body);
    Ok(response)
  }
}
