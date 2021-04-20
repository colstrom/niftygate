pub mod prelude {
  pub use tide::http::{headers::HeaderName, StatusCode, Url};
  pub use web3::{transports::WebSocket, Web3};
}

use prelude::*;

use std::result;
use tide::{utils::async_trait, Middleware, Next, Request, Response, Result};
use web3::types::{Address, Recovery};

#[derive(Clone)]
pub struct ProvidesAccountVerification {
  pub signature_header: HeaderName,
  pub account_header: HeaderName,
  pub status_code: StatusCode,
  pub web3: Web3<WebSocket>,
  pub message: Vec<u8>,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ProvidesAccountVerification {
  async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> Result {
    let mut accounts: Vec<web3::types::H160> = vec![];

    match request.header(&self.signature_header) {
      None => return Ok(Response::new(self.status_code)),
      Some(header_values) => match header_values
        .into_iter()
        .map(|value| base64::decode(value.as_str()))
        .collect::<result::Result<Vec<Vec<u8>>, _>>()
      {
        Err(_) => return Ok(Response::new(StatusCode::BadRequest)),
        Ok(raw_signatures) => match raw_signatures
          .into_iter()
          .map(|raw_signature| Recovery::from_raw_signature(self.message.clone(), raw_signature))
          .collect::<result::Result<Vec<Recovery>, _>>()
        {
          Err(_) => return Ok(Response::new(StatusCode::UnsupportedMediaType)),
          Ok(recovery_messages) => match recovery_messages
            .into_iter()
            .map(|recovery| self.web3.accounts().recover(recovery))
            .collect::<result::Result<Vec<Address>, _>>()
          {
            Err(_) => return Ok(Response::new(StatusCode::UnprocessableEntity)),
            Ok(recovered_accounts) => {
              for account in recovered_accounts {
                accounts.push(account)
              }
            }
          },
        },
      },
    }

    for account in accounts {
      request.append_header(&self.account_header, base64::encode(account))
    }

    Ok(next.run(request).await)
  }
}
