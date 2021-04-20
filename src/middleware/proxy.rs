pub mod prelude {
  pub use surf::Client;
  pub use tide::http::{headers, headers::HeaderName, Url};
}

use prelude::*;

use tide::{http, utils::async_trait, Middleware, Next, Request, Result};

#[derive(Clone)]
pub struct Proxy {
  pub client: Client,
  pub backend: Url,
}

impl Proxy {
  pub fn new(backend: Url) -> Self {
    Self {
      client: surf::client(),
      backend,
    }
  }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for Proxy {
  async fn handle(&self, request: Request<State>, _next: Next<'_, State>) -> Result {
    let mut request: http::Request = request.into();
    let url = request.url_mut();
    url.set_host(self.backend.host_str())?;
    url.set_port(self.backend.port_or_known_default()).unwrap();
    url.set_scheme(self.backend.scheme()).unwrap();

    let mut response: http::Response = surf::client().send(request).await?.into();
    response.remove_header(headers::CONNECTION);
    Ok(response.into())
  }
}
