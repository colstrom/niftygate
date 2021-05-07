pub mod prelude {
  pub use ethcontract::web3::{
    transports::WebSocket,
    types::{Address, Recovery},
    Web3,
  };
  pub use tide::http::{headers::HeaderName, StatusCode, Url};
}

use prelude::*;

use std::result;
use tide::{utils::async_trait, Middleware, Next, Request, Response, Result};

#[derive(Clone)]
pub struct ProvidesAccountVerification {
  pub address_header: HeaderName,
  pub challenge: Vec<u8>,
  pub signature_header: HeaderName,
  pub status_code: StatusCode,
  pub web3: Web3<WebSocket>,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ProvidesAccountVerification {
  async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> Result {
    let mut addresses: Vec<Address> = vec![];

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
          .map(|raw_signature| Recovery::from_raw_signature(self.challenge.clone(), raw_signature))
          .collect::<result::Result<Vec<Recovery>, _>>()
        {
          Err(_) => return Ok(Response::new(StatusCode::UnsupportedMediaType)),
          Ok(recovery_messages) => match recovery_messages
            .into_iter()
            .map(|recovery| self.web3.accounts().recover(recovery))
            .collect::<result::Result<Vec<Address>, _>>()
          {
            Err(_) => return Ok(Response::new(StatusCode::UnprocessableEntity)),
            Ok(recovered) => {
              for address in recovered {
                addresses.push(address)
              }
            }
          },
        },
      },
    }

    for address in addresses {
      request.append_header(&self.address_header, hex::encode(address))
    }

    Ok(next.run(request).await)
  }
}
