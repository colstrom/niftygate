use crate::application::demo;
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use tide::log;
use tide_rustls::TlsListener;

#[derive(Debug, StructOpt)]
#[structopt(about = "Runs a sample app that returns request headers in the response body")]
pub struct Command {
  #[structopt(
    env,
    long,
    short,
    value_name = "address",
    default_value = "127.0.0.1:8080"
  )]
  listen: String,

  #[structopt(
    env,
    long,
    takes_value = false,
    requires = "tls-certificate-path",
    requires = "tls-key-path"
  )]
  with_tls: bool,

  #[structopt(env, long, value_name = "Path", requires = "tls-key-path")]
  tls_certificate_path: Option<PathBuf>,

  #[structopt(env, long, value_name = "Path", requires = "tls-certificate-path")]
  tls_key_path: Option<PathBuf>,
}

impl Command {
  pub async fn execute(self) -> Result<()> {
    log::with_level(log::LevelFilter::Debug);

    let server = demo::server();

    if self.with_tls {
      match (self.tls_certificate_path, self.tls_key_path) {
        (Some(tls_certificate_path), Some(tls_key_path)) => {
          server.listen(TlsListener::build()
            .addrs(&self.listen)
            .cert(&tls_certificate_path)
            .key(&tls_key_path)).await?
          },
          _ => panic!("Missing either certificate or key. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻"),
        }
    } else {
      server.listen(&self.listen).await?
    }

    Ok(())
  }
}
