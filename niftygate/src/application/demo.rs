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
