#[async_std::main]
async fn main() -> std::io::Result<()> {
  Ok(
    sig_proxy::application::demo::server()
      .listen("127.0.0.1:8000")
      .await?,
  )
}
