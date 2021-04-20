use tide::{Request, Result};

pub async fn endpoint<State: Clone + Send + Sync + 'static>(
  request: Request<State>,
) -> Result<String> {
  Ok(
    request
      .header_names()
      .map(|key| format!("{}: {}", key, request.header(key).unwrap()))
      .collect::<Vec<String>>()
      .join("\n"),
  )
}

pub fn server() -> tide::Server<()> {
  let mut server = tide::new();
  server.at("/").get(endpoint);
  server
}

use tide::log;

#[async_std::main]
async fn main() -> std::io::Result<()> {
  log::with_level(log::LevelFilter::Debug);
  server().listen("127.0.0.1:8000").await?;
  Ok(())
}
