use sig_proxy::WrappedResult;

#[async_std::main]
async fn main() -> WrappedResult<()> {
  sig_proxy::command::run().await
}
