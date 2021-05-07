pub mod prelude {
  pub use tide::http::{headers, headers::HeaderName, StatusCode};
}

use prelude::*;

use tide::{
  http::proxies::Forwarded, utils::async_trait, Middleware, Next, Request, Response, Result,
};

const UNKNOWN: &str = "unknown";

fn forwarded<State: Clone + Send + Sync + 'static>(request: &Request<State>) -> Forwarded {
  let mut forwarded = Forwarded::new();
  forwarded.add_for(request.peer_addr().unwrap_or(UNKNOWN));
  forwarded.set_by(request.local_addr().unwrap_or(UNKNOWN));
  forwarded.set_proto(request.url().scheme());
  forwarded.set_host(match request.header(headers::HOST) {
    Some(host) => host.as_str(),
    None => UNKNOWN,
  });
  forwarded
}

#[derive(Clone)]
pub struct ProvidesForwardedHeader;

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ProvidesForwardedHeader {
  async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> Result {
    request.append_header(headers::FORWARDED, forwarded(&request).value()?);

    Ok(next.run(request).await)
  }
}

#[derive(Clone)]
pub struct RequiresHeaders {
  pub headers: Vec<HeaderName>,
  pub status: StatusCode,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequiresHeaders {
  async fn handle(&self, request: Request<State>, next: Next<'_, State>) -> Result {
    for header in &self.headers {
      if request.header(header).is_none() {
        return Ok(Response::new(self.status));
      }
    }

    Ok(next.run(request).await)
  }
}

#[derive(Clone)]
pub struct RemovesHeaders {
  headers: Vec<HeaderName>,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RemovesHeaders {
  async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> Result {
    for header in &self.headers {
      request.remove_header(header);
    }

    Ok(next.run(request).await)
  }
}
