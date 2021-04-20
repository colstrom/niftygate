pub mod prelude {
  pub use tide::http::{headers, headers::HeaderName, StatusCode};
}

use prelude::*;

use tide::{utils::async_trait, Middleware, Next, Request, Response, Result};

#[derive(Clone)]
pub struct RequiresAuthorization {
  pub authorization: HeaderName,
  pub authenticate: HeaderName,
  pub status: StatusCode,
  pub scheme: String,
}

impl RequiresAuthorization {
  pub fn www(scheme: String) -> Self {
    Self {
      authorization: headers::AUTHORIZATION,
      authenticate: headers::WWW_AUTHENTICATE,
      status: StatusCode::Unauthorized,
      scheme,
    }
  }

  pub fn proxy(scheme: String) -> Self {
    Self {
      authorization: headers::PROXY_AUTHORIZATION,
      authenticate: headers::PROXY_AUTHENTICATE,
      status: StatusCode::ProxyAuthenticationRequired,
      scheme,
    }
  }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequiresAuthorization {
  async fn handle(&self, request: Request<State>, next: Next<'_, State>) -> Result {
    match request.header(&self.authorization) {
      Some(_) => Ok(next.run(request).await),
      None => {
        let mut response = Response::new(self.status);
        response.insert_header(&self.authenticate, &self.scheme);
        Ok(response)
      }
    }
  }
}
