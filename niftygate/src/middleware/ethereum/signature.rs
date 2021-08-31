pub mod prelude {
  pub use ethcontract::{
    dyns::DynWeb3,
    web3::{transports::WebSocket, Web3},
  };
  pub use secp256k1::SecretKey;
  pub use tide::http::{headers::HeaderName, Url};
}

use prelude::*;

use ethcontract::web3::signing::SecretKeyRef;
use tide::{utils::async_trait, Middleware, Next, Request, Result};

#[derive(Clone)]
pub struct ProvidesSignature {
  pub challenge: Vec<u8>,
  pub secret_key: SecretKey,
  pub signature_header: HeaderName,
  pub web3: DynWeb3,
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ProvidesSignature {
  async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> Result {
    let key = SecretKeyRef::new(&self.secret_key);
    let signed_data = self.web3.accounts().sign(&self.challenge, key);
    let signature = signed_data.signature.0;

    request.append_header(&self.signature_header, base64::encode(signature));

    Ok(next.run(request).await)
  }
}
