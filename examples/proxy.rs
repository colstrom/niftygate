use niftygate::{middleware::prelude::*, middleware::*};
use tide::log;

type WrappedError = Box<dyn std::error::Error>;

#[async_std::main]
async fn main() -> std::result::Result<(), WrappedError> {
  log::with_level(log::LevelFilter::Debug);

  let mut server = tide::new();
  server
    .with(ProvidesForwardedHeader)
    .with(RequiresAuthorization::www(String::from("Basic")))
    .with(Proxy::new(Url::parse("http://127.0.0.1:8000")?));
  server.listen("127.0.0.1:8001").await?;

  Ok(())
}
