use super::Downloader;
use crate::prelude::http::{Error, Response, Url};
use async_trait::async_trait;

#[derive(Clone, Debug, Default)]
pub struct SurfDownloader {
  client: surf::Client,
}

#[async_trait]
impl Downloader for SurfDownloader {
  async fn download(&self, url: &Url) -> Result<Response, Error> {
    Ok(self.client.get(url).await?.into())
  }
}
