use crate::prelude::http::{Error, Response, Url};
use async_trait::async_trait;

#[async_trait]
pub trait Downloader
where
  Self: Send + Sync,
{
  async fn download(&self, url: &Url) -> Result<Response, Error>;
}
