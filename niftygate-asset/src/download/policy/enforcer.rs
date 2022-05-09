// use super::super::Downloader;
// use crate::prelude::http::{Error, Response, StatusCode, Url};
// use async_trait::async_trait;
// use policies::Policy;
// use policies::UriPolicyLogic;
// use std::convert::TryFrom;
// use uriparse::URI;

// #[derive(Debug, thiserror::Error)]
// pub enum DownloadPolicyError {
//   #[error("request denied by download policy")]
//   Denied,
// }

// /// A wrapper around any downloader adding policy support.
// pub struct DownloadPolicyEnforcer<D>
// where
//   D: Downloader,
// {
//   downloader: D,
//   policies: Vec<UriPolicyLogic>,
// }

// impl<D> DownloadPolicyEnforcer<D>
// where
//   D: Downloader,
// {
//   pub fn new(downloader: D) -> Self {
//     Self::new_with_policies(downloader, Vec::new())
//   }

//   pub fn new_with_policies(downloader: D, policies: Vec<UriPolicyLogic>) -> Self {
//     Self {
//       downloader,
//       policies,
//     }
//   }

//   pub fn add_policy<T>(&mut self, policy: T)
//   where
//     T: Into<UriPolicyLogic>,
//   {
//     self.policies.push(policy.into())
//   }

//   pub fn permits_uri(&self, uri: &URI) -> bool {
//     self.policies.iter().all(|policy| policy.evaluate(uri))
//   }

//   pub fn permits_url(&self, url: &Url) -> bool {
//     URI::try_from(url.as_str()).map_or(false, |uri| self.permits_uri(&uri))
//   }
// }

// #[async_trait]
// impl<D> Downloader for DownloadPolicyEnforcer<D>
// where
//   D: Downloader,
// {
//   async fn download(&self, url: &Url) -> Result<Response, Error> {
//     if self.permits_url(url) {
//       self.downloader.download(url).await
//     } else {
//       Err(Error::new(
//         StatusCode::Forbidden,
//         DownloadPolicyError::Denied,
//       ))
//     }
//   }
// }
